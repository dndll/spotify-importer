use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Error};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::cli::Opts;
use crate::provider::StreamingProvider;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeData {
    pub contents: Contents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_watch_next_results: TwoColumnWatchNextResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnWatchNextResults {
    pub playlist: Playlist,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub playlist: Playlist2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist2 {
    pub contents: Vec<Content>,
    pub total_videos: i64,
    pub owner_name: OwnerName,
    pub playlist_share_url: String,
    pub endpoint: Endpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub playlist_panel_video_renderer: PlaylistPanelVideoRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoRenderer {
    pub title: Title,
    pub length_text: LengthText,
    pub navigation_endpoint: NavigationEndpoint2,
    pub video_id: String,
    pub short_byline_text: ShortBylineText,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub accessibility: Accessibility,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub accessibility_data: AccessibilityData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LengthText {
    pub accessibility: Accessibility,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata2,
    pub watch_endpoint: WatchEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata2 {
    pub web_command_metadata: WebCommandMetadata2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata2 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
    pub playlist_id: String,
    pub index: i64,
    pub params: String,
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
    pub navigation_endpoint: NavigationEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint3 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata3,
    pub browse_endpoint: BrowseEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata3 {
    pub web_command_metadata: WebCommandMetadata3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata3 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint2 {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlay {
    pub thumbnail_overlay_time_status_renderer: Option<ThumbnailOverlayTimeStatusRenderer>,
    pub thumbnail_overlay_toggle_button_renderer: Option<ThumbnailOverlayToggleButtonRenderer>,
    pub thumbnail_overlay_now_playing_renderer: Option<ThumbnailOverlayNowPlayingRenderer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayTimeStatusRenderer {
    pub text: Text,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub accessibility: Accessibility3,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility3 {
    pub accessibility_data: AccessibilityData3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData3 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayToggleButtonRenderer {
    pub is_toggled: bool,
    pub untoggled_icon: UntoggledIcon,
    pub toggled_icon: ToggledIcon,
    pub untoggled_tooltip: String,
    pub toggled_tooltip: String,
    pub untoggled_service_endpoint: UntoggledServiceEndpoint,
    pub toggled_service_endpoint: ToggledServiceEndpoint,
    pub untoggled_accessibility: UntoggledAccessibility,
    pub toggled_accessibility: ToggledAccessibility,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledIcon {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledIcon {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata4,
    pub playlist_edit_endpoint: PlaylistEditEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata4 {
    pub web_command_metadata: WebCommandMetadata4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata4 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEditEndpoint {
    pub playlist_id: String,
    pub actions: Vec<Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub added_video_id: String,
    pub action: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata5,
    pub playlist_edit_endpoint: PlaylistEditEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata5 {
    pub web_command_metadata: WebCommandMetadata5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata5 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEditEndpoint2 {
    pub playlist_id: String,
    pub actions: Vec<Action2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action2 {
    pub action: String,
    pub removed_video_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledAccessibility {
    pub accessibility_data: AccessibilityData4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData4 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledAccessibility {
    pub accessibility_data: AccessibilityData5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData5 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayNowPlayingRenderer {
    pub text: Text2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub runs: Vec<Run3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run3 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerName {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata6,
    pub browse_endpoint: BrowseEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata6 {
    pub web_command_metadata: WebCommandMetadata6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata6 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint3 {
    pub browse_id: String,
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
impl StreamingProvider<YoutubeData> for YoutubeProvider {
    //TODO should be response dto from reading a csv of raws
    async fn gather_data(&self) -> Result<YoutubeData, anyhow::Error> {
        println!("> Retrieving youtube data from url..");
        println!("> Extracting store dump..");
        println!("> Deserialising initial data..");
        let data = get_raws_from_file()?; //TODO start retrieving the data
        let total_videos = data.contents.two_column_watch_next_results.playlist.playlist.total_videos;
        println!("> Importing {} tracks..", total_videos);
        Ok(data)
    }

    fn convert_to_query(&self, item: YoutubeData) -> Vec<(String, String)> {
        let contents = item.contents.two_column_watch_next_results.playlist.playlist.contents;
        contents.iter().map(|content| {
            let title = &content.playlist_panel_video_renderer.title.simple_text;
            determine_artist_from_title(title).unwrap()
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
    let mut song_replaced = if song.contains("[") {
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

pub fn get_raws_from_file() -> Result<YoutubeData, Error> {
    let rdr = File::open("./ytInitialData.json")?;
    Ok(serde_json::from_reader(rdr)?)
}