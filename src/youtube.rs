use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Error};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::cli::Opts;
use crate::provider::StreamingProvider;
use serde_json::Value;
use json_dotpath::DotPaths;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeData {
    pub contents: Contents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tab>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub tab_renderer: TabRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabRenderer {
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRenderer {
    pub contents: Vec<Content2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub item_section_renderer: ItemSectionRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer {
    pub contents: Vec<Content3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    pub playlist_video_list_renderer: PlaylistVideoListRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoListRenderer {
    pub contents: Vec<Content4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content4 {
    pub playlist_video_renderer: Option<PlaylistVideoRenderer>,
    pub continuation_item_renderer: Option<ContinuationItemRenderer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoRenderer {
    pub title: Title,
    pub short_byline_text: ShortBylineText,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub runs: Vec<Run>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortBylineText {
    pub runs: Vec<Run2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run2 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata,
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemRenderer {
    pub trigger: String,
    pub continuation_endpoint: ContinuationEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata2,
    pub continuation_command: ContinuationCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata2 {
    pub web_command_metadata: WebCommandMetadata2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata2 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationCommand {
    pub token: String,
    pub request: String,
}



#[derive(Default, Debug, Clone)]
pub struct YoutubeProvider {
    pub playlist: String
}


impl YoutubeProvider {
    pub fn new(opts: &Opts) -> YoutubeProvider {
        YoutubeProvider {
            playlist: opts.youtube_playlist.to_owned()
        }
    }
}


#[async_trait]
impl StreamingProvider<PlaylistVideoListRenderer> for YoutubeProvider {
    //TODO should be response dto from reading a csv of raws
    async fn gather_data(&self) -> Result<PlaylistVideoListRenderer, anyhow::Error> {
        println!("> Retrieving youtube data from url..");
        println!("> Extracting store dump..");
        println!("> Deserialising initial data..");
        let data = get_raws_from_file()?; //TODO start retrieving the data
        let total_videos = data.contents.len();
        println!("> Importing {} tracks..", total_videos);
        Ok(data)
    }

    fn convert_to_query(&self, item: PlaylistVideoListRenderer) -> Vec<(String, String)> {
        let contents = item.contents;
        contents.iter().map(|content| {
            let renderer = &content.playlist_video_renderer;
            let title = if renderer.is_some() {
                renderer.as_ref().unwrap().title.runs.first().unwrap().text.to_lowercase()
            } else {
                String::from("empty - empty")
            };
            determine_artist_from_title(&title).unwrap()
        }).collect()
    }

    async fn build_queries(&self) -> Result<Vec<(String, String)>, anyhow::Error> {
        let provider = self.gather_data().await?;
        let queries = self.convert_to_query(provider);
        Ok(queries)
    }
}

fn determine_artist_from_title(title: &String) -> Result<(String, String), Error> {
    let array: Vec<&str> = title.split("-").collect();
    let mut artist = array.first().context("Failed to get artist")?.to_lowercase();
    if artist.ends_with(" ") {
        artist.pop();
    }

    let song = array.get(1).context("Failed to get artist")?;
    let song_replaced = if song.contains("[") {
        let mut song_replaced = song
            .split("[")
            .collect::<Vec<&str>>()
            .first()
            .context("Failed to get song")?.to_lowercase();
        if song_replaced.ends_with(" ") {
            song_replaced = song_replaced.trim_end().to_lowercase()
        }
        if song_replaced.starts_with(" ") {
            song_replaced = song_replaced.trim_start().to_lowercase()
        }
        song_replaced
    } else {
        song.to_lowercase()
    };
    Ok((artist, song_replaced))
}

pub fn get_raws_from_file() -> Result<PlaylistVideoListRenderer, Error> {
    let rdr = File::open("./ytInitialData.json")?;
    let res: Value = serde_json::from_reader(rdr)?;
    let data: PlaylistVideoListRenderer = res.dot_get("contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.playlistVideoListRenderer")?.expect("Failed to read for renderer");
    let continuation: Content4 = res.dot_get("contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.playlistVideoListRenderer.contents.>")?.expect("Failed to read command contents");
    let token = continuation.continuation_item_renderer;
    println!("Token is {:?}", token);
    // let res = (vec.dot_get::<Value>("0.0.1.4")
    Ok(data)
}