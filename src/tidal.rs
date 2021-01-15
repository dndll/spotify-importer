use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Error;
use async_trait::async_trait;
use rspotify::model::{search::SearchResult, track::FullTrack};
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::senum::{Country, IncludeExternal, SearchType};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::Opts;
use crate::provider::StreamingProvider;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tidal {
    pub limit: i64,
    pub offset: i64,
    pub total_number_of_items: i64,
    pub items: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub item: TrackDetails,
    #[serde(rename = "type")]
    pub type_field: String,
    pub cut: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackDetails {
    pub id: i64,
    pub title: String,
    pub duration: i64,
    pub replay_gain: f64,
    pub peak: f64,
    pub allow_streaming: bool,
    pub stream_ready: bool,
    pub stream_start_date: String,
    pub premium_streaming_only: bool,
    pub track_number: i64,
    pub volume_number: i64,
    pub version: Option<String>,
    pub popularity: i64,
    pub copyright: String,
    pub description: Value,
    pub url: String,
    pub isrc: String,
    pub editable: bool,
    pub explicit: bool,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    pub artist: Artist,
    pub artists: Vec<Artist2>,
    pub album: Album,
    pub mixes: Mixes,
    pub date_added: String,
    pub index: i64,
    pub item_uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist2 {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub video_cover: Value,
    pub release_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mixes {
    #[serde(rename = "TRACK_MIX")]
    pub track_mix: Option<String>,
    #[serde(rename = "MASTER_TRACK_MIX")]
    pub master_track_mix: Option<String>,
}


#[derive(Default, Debug, Clone)]
pub struct TidalProvider {
    pub playlist: String,
    pub file: PathBuf,
}

impl TidalProvider {
    pub fn new(opts: &Opts) -> TidalProvider {
        TidalProvider {
            playlist: opts.playlist.to_owned(),
            file: opts.tidal_file.as_ref().expect("Failed to unwrap the tidal_file parameter").to_path_buf(),
        }
    }
}

#[async_trait]
impl StreamingProvider<Tidal> for TidalProvider {
    async fn gather_data(&self) -> Result<Tidal, Error> {
        println!("> Reading tidal file..");
        let tidal = get_tidal_from_file(&self.file).await?;
        // read a source
        println!("> Importing {} tracks..", tidal.items.len() - 1);
        Ok(tidal)
    }

    fn convert_to_query(&self, item: Tidal) -> Vec<(String, String)> {
        println!("> Converting to query..");
        // convert items to title with artists
        item.items.iter()
            .map(|track| {
                let artist: String = track.item.artists.iter().map(|artist| artist.name.to_lowercase()).collect::<Vec<String>>().join(" ");
                let query: String = vec![artist, track.item.title.to_lowercase()].join(" ");
                (track.item.artist.name.to_lowercase(), query)
            }).collect()
    }

    async fn build_queries(&self) -> Result<Vec<(String, String)>, Error> {
        let tidal = self.gather_data().await?;
        let queries = self.convert_to_query(tidal);
        Ok(queries)
    }
}

pub async fn get_tidal_from_file(path: &PathBuf) -> Result<Tidal, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: Result<Tidal, _> = serde_json::from_reader(reader);
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(anyhow::anyhow!(format!("Some issue {}", err)))
    }
}
