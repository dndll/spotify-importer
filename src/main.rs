use anyhow::Error;
use futures::{SinkExt, StreamExt};
use rspotify::client::Spotify;
use rspotify::model::cud_result::CUDResult;
use rspotify::model::search::SearchResult;
use rspotify::model::track::FullTrack;
use rspotify::model::user::PrivateUser;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::senum::{Country, IncludeExternal, SearchType};
use rspotify::util::get_token;

use cli::Opts;

use crate::cli::get_opts_args;
use crate::provider::StreamingProvider;
use crate::tidal::TidalProvider;

mod tidal;
mod cli;
mod provider;

enum Platform {
    TIDAL,
    NONE,
}

impl Platform {
    fn get_enum_from_string(value: String) -> Platform {
        let value = value.to_lowercase();
        let value = value.as_str();
        match value {
            "tidal" => Platform::TIDAL,
            _ => Platform::NONE
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let opts = get_opts_args();

    // TODO: hone down scope of app before deploying
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-recently-played playlist-modify-public playlist-modify-private user-follow-read user-follow-modify playlist-modify-private user-library-modify user-library-read")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let platform = Platform::get_enum_from_string(opts.platform.clone());
            let (spotify, user) = get_spotify(token_info).await;
            let (provider, queries) = match platform {
                Platform::TIDAL => {
                    let provider = TidalProvider::new(&opts);
                    Ok((provider.clone(), provider.build_queries().await?))
                }
                Platform::NONE => Err(anyhow::anyhow!("We do not support platform with options {}", opts.platform))
            }?;

            // search for tracks (artist, concat of artists and track title)
            println!("> Searching tracks..");
            let mut search_results: Vec<(String, String, Result<SearchResult, _>)> = vec![];


            for (artist, query) in queries {
                let query = sanitize_query(query);
                let query_cloned = query.clone();
                let future = spotify.search(
                    query_cloned.as_str(),
                    SearchType::Track,
                    50,
                    0,
                    None,
                    None,
                );
                search_results.push((artist, query, future.await));
            }

            let mut track_uris = vec![];
            let mut failed_uris = vec![];

            //TODO maybe use par it
            search_results.iter()
                .for_each(|(artist, query, find)| {
                    if let Ok(SearchResult::Tracks(tracks)) = find {
                        let tracks = tracks.items
                            .iter()
                            .filter(|track| {
                                let artists = build_track_artists(track);
                                artists.contains(&artist)
                            }).collect::<Vec<&FullTrack>>();
                        match tracks.first() {
                            None => {
                                let message = format!("Could not find {} {}", artist, query);
                                failed_uris.push(message);
                            }
                            Some(value) => {
                                let uri = value.uri.clone();
                                log::debug!("Found {} {:?}", query, uri);
                                track_uris.push(uri);
                            }
                        }
                    }
                });

            failed_uris.iter().for_each(|message| log::debug!("{}", message));


            let mut results = vec![];
            //TODO at this point we should probably retry
            let mut futures = track_uris.chunks(80);
            while let Some(track_ids) = futures.next() {
                results.push(spotify.user_playlist_add_tracks(
                    user.id.as_str(),
                    provider.playlist.as_str(),
                    &track_ids,
                    None,
                ).await);
            }

            results.iter().for_each(|res| {
                match res {
                    Ok(result) => println!("Added {:?}", result),
                    Err(err) => println!("Failed to add because {}", err),
                }
            });

            //TODO dont do this
            Ok(())

        }
        None => Err(anyhow::anyhow!("Authentication failed, have you set up your .env file?")),
    }
}

async fn get_spotify(token_info: TokenInfo) -> (Spotify, PrivateUser) {
    log::debug!("> Getting spotify credentials..");
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    println!("> Getting user..");
    let user = spotify.current_user().await.expect("Failed to get user");

    (spotify, user)
}


fn sanitize_query(query: String) -> String {
    let query = query.replace("(feat. ", "");
    let query = query.replace(")", "");
    query
}


fn build_track_artists(track: &FullTrack) -> Vec<String> {
    track.artists.iter().map(|artist| artist.name.to_lowercase()).collect::<Vec<String>>()
}