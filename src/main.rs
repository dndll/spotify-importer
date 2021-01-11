use anyhow::Error;
use raw::RawProvider;
use rspotify::client::Spotify;
use rspotify::model::search::SearchResult;
use rspotify::model::track::FullTrack;
use rspotify::model::user::PrivateUser;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::senum::{SearchType};
use rspotify::util::get_token;
use anyhow::anyhow;

use crate::cli::get_opts_args;
use crate::provider::StreamingProvider;
use crate::tidal::TidalProvider;
use std::str::FromStr;

mod tidal;
mod cli;
mod provider;
mod raw;
#[derive(Debug)]
pub enum Platform {
    TIDAL,
    NONE,
    RAW,
}
impl FromStr for Platform {
    type Err = Error;
    fn from_str(day: &str) -> Result<Self, Error> {
        match day {
            "tidal" => Ok(Platform::TIDAL),
            "raw" => Ok(Platform::RAW),
            _ => Err(anyhow!("Could not parse a platform")),
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
            let (spotify, user) = get_spotify(token_info).await;
            let queries = match opts.platform {
                Platform::TIDAL => {
                    let provider = TidalProvider::new(&opts);
                    Ok(provider.build_queries().await?)
                },
                Platform::NONE => Err(anyhow::anyhow!("We do not support platform with options {:?}", opts.platform)),
                Platform::RAW => {
                    let provider = RawProvider::new(&opts);
                    Ok(provider.build_queries().await?)
                }
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
                    10,
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
                    opts.playlist.as_str(),
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