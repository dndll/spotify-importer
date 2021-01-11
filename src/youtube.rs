use std::fs::File;

use anyhow::{Context, Error};
use async_trait::async_trait;
use json_dotpath::DotPaths;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

        let mut data_list = vec![];
        let mut token_counter = String::from("1");
        let mut started = false;
        while !token_counter.is_empty() {
            println!("started {} token counter {}", started, token_counter);
            let json = if !started {

                println!("> Retrieving initial youtube data..");
                let html = retrieve_youtube_data(&self.playlist).await?;
                println!("> Extracting store dump..");
                extract_initial_yt_data(html)?
            } else {
                println!("> Retrieving next page..");
                retrieve_next_page(token_counter).await?
            };



            println!("> Deserialising data..");
            let value: Value = serde_json::from_str(json.trim_end())?;

            if !started {
                let token: Content4 = value.dot_get("contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.playlistVideoListRenderer.contents.>")?.expect("Failed to read command contents");
                let token = token.continuation_item_renderer.unwrap().continuation_endpoint.continuation_command.token;
                token_counter = token;
            } else {
                let token: Content4 = value.dot_get("onResponseReceivedActions.0.appendContinuationItemsAction.continuationItems.>")?.expect("Failed to read command contents");
                let renderer = token.continuation_item_renderer;
                match renderer {
                    None => {
                        token_counter = String::new()
                    }
                    Some(renderer) => {
                        let token = renderer.continuation_endpoint.continuation_command.token;
                        token_counter = token;
                    }
                }
            }


            let mut data = if !started {
                extract_initial_data(value)?
            } else {
                extract_data(value)?
            };
            data_list.append(&mut data.contents);

            started = true;

        }

        let total_videos = data_list.len();
        println!("> Importing {} tracks..", total_videos);
        Ok(PlaylistVideoListRenderer { contents: data_list })
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

fn determine_artist_from_title(title: &str) -> Result<(String, String), Error> {
    println!("determining artist for title: {}", title);
    let (mut artist, song) = if title.contains('-') {
        let array: Vec<&str> = title.split('-').collect();
        (array.first().context("Failed to get artist")?.to_lowercase(), array.get(1).context("Failed to get artist")?.to_string())
    } else {
        (String::new(), title.to_string())
    };
    if artist.ends_with(' ') {
        artist.pop();
    }

    let song_replaced = if song.contains('[') {
        let mut song_replaced = song
            .split("[")
            .collect::<Vec<&str>>()
            .first()
            .context("Failed to get song")?.to_lowercase();
        if song_replaced.ends_with(' ') {
            song_replaced = song_replaced.trim_end().to_lowercase()
        }
        if song_replaced.starts_with(' ') {
            song_replaced = song_replaced.trim_start().to_lowercase()
        }
        song_replaced
    } else {
        song.to_lowercase()
    };
    Ok((artist, song_replaced))
}

fn extract_initial_yt_data(html: String) -> Result<String, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("var ytInitialData = (.*);").unwrap();
    }
    let mut result: String = RE.find_iter(&html)
        .map(|mat| mat.as_str())
        .collect::<String>()
        .replace("var ytinitialdata = ", "");
    let range = result.find(';').unwrap_or(result.len());
    result.replace_range(range.., "");
    Ok(result.replace("var ytInitialData = ", ""))
}

fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("/var ytInitialData = (.*);").unwrap();
    }
    RE.is_match(text)
}

async fn retrieve_youtube_data(playlist: &String) -> Result<String, Error> {
    let response = reqwest::get(&build_playlist_url(&playlist))
        .await?
        .text()
        .await?;

    Ok(response)
}

async fn retrieve_next_page(token: String) -> Result<String, Error> {
    // lazy_static! {
    //     static ref CLIENT: Client = reqwest::Client::new();
    // }
    let client = reqwest::Client::builder();
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:85.0) Gecko/20100101 Firefox/85.0".parse().unwrap());
    header_map.insert("X-Goog-Visitor-Id", "CgtEbWNCbGdTZXVmOCjh-PD_BQ%3D%3D".parse().unwrap());
    header_map.insert("X-Youtube-Client-Name", "1".parse().unwrap());
    header_map.insert("X-Youtube-Client-Version", "2.20210107.08.00".parse().unwrap());
    header_map.insert("Authorization", "SAPISIDHASH 1610366180_d5ae2bad7a2c4b0636d3ac393689e8ac951c98ad".parse().unwrap());
    header_map.insert("X-Goog-AuthUser", "0".parse().unwrap());
    header_map.insert("X-Origin", "https://www.youtube.com".parse().unwrap());
    header_map.insert("Origin", "https://www.youtube.com".parse().unwrap());
    header_map.insert("Cookie", "VISITOR_INFO1_LIVE=DmcBlgSeuf8; PREF=al=en-GB; CONSENT=YES+GB.en-GB+V9; __Secure-3PSID=5gdlMYNkjtKXm7vkbvrMVVwpvPZ2Jd4axg319K1khe-HUl39-tNPBt59MEWIO7-cdVVNeQ.; __Secure-3PAPISID=KDyp_Pvxmizot43f/A9ACfC9X5qr9oEvoe; __Secure-3PSIDCC=AJi4QfESnWYtKI6KxaudytOC7qeUChk6REaOMlxCEoJH_RLjgkJf-tGYDhsYnJrKmhMnaAnlP3s; SID=5gdlMYNkjtKXm7vkbvrMVVwpvPZ2Jd4axg319K1khe-HUl39QyJyQYAtI7_52p0yJgypJQ.; HSID=AwBtT9160yU0wYL89; SSID=AygN54p0g04YMbeE5; APISID=Py38xqaJCSgTAOY0/Al8eBKTBByIt_h8lf; SAPISID=KDyp_Pvxmizot43f/A9ACfC9X5qr9oEvoe; SIDCC=AJi4QfHRyCzmsTmacJAko-gVWpEeuNLnufE8nKaWWcVON21Fz6OP05rzd5Aq1Y-fc4OhSyqtNVk; YSC=f9EGCfPk2i4; LOGIN_INFO=AFmmF2swRgIhAPIxipPOt5Zs9hO6I5roY2K7eqTeN-uLQW2fsuqIlIV4AiEA4iRD7lTD7Wr_WlVoQtIoMhNtWQlwMxuAnKxse84lDJg:QUQ3MjNmd21VRWRsOVcwX1JVRXhHUDdmY2ZINUJpZTN4dk9ld3FTT2JpWXhuVTY2S0gwMEJBOGZqdEVaQUx5LVlrZGd0R3JCWHZNVHYyYV9VWF9PMjM2Tm5La2VOZDBKSUhJYkFhNE11YklkQUF2amtZVXVqR2oyOHNyblFoV3dvR3J5bjJUbGpMLUJhX2E0NDZHMEhDUWJXRU0xUWtMdHh3TG1iWGhiZVlwV2NIenJhZ3hCS21pLWZWOVJ1STJ0V2dkUFdZcVhyTHc3".parse().unwrap());
    header_map.insert("TE", "Trailers".parse().unwrap());
    // header_map.insert("", "".parse().unwrap());
    let client = client.default_headers(header_map).build()?;


    let res = client.post("https://www.youtube.com/youtubei/v1/browse")
        .query(&[("key", "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8")])
        .body(build_next_request(token))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

fn build_next_request(token: String) -> String {
    let value = serde_json::json!({
    "context": {
        "client": {
            "hl": "en-GB",
            "gl": "GB",
            "geo": "GB",
            "remoteHost": "77.102.111.53",
            "isInternal": true,
            "deviceMake": "",
            "deviceModel": "",
            "visitorData": "CgtEbWNCbGdTZXVmOCjh-PD_BQ%3D%3D",
            "userAgent": "Mozilla/5.0 (X11; Linux x86_64; rv:85.0) Gecko/20100101 Firefox/85.0,gzip(gfe)",
            "clientName": "WEB",
            "clientVersion": "2.20210107.08.00",
            "osName": "X11",
            "osVersion": "",
            "originalUrl": "https://www.youtube.com/playlist?list=UUTZ35GQfSb0RsPdREhWZrtg",
            "internalClientExperimentIds": [
                44496012
            ],
            "platform": "DESKTOP",
            "gfeFrontlineInfo": "vip=216.58.204.238,server_port=443,client_port=42268,tcp_connection_request_count=0,header_order=HUALEC,gfe_version=2.699.14,ssl,ssl_info=TLSv1.3:RA:F,tlsext=S,sni=www.youtube.com,hex_encoded_client_hello=130113031302c02bc02fcca9cca8c02cc030c00ac009c013c014009c009d002f0035000a-00-00000017ff01000a000b001000050033002b000d002d001c0029,c=1301,pn=alpn,ja3=df208241e7f3897d4ca38cfe68eabb21,rtt_source=h2_ping,rtt=18,srtt=30,client_protocol=h2,client_transport=tcp,gfe=aclhrpp16.prod.google.com,pzf=Linux 2.2.x-3.x [4:56+8:0:1460:mss*44/7:mss/sok/ts/nop/ws:df/id+:0] [generic tos:0x20],vip_region=default,asn=5089,cc=GB,eid=YDz8X8H0PJDx8Aech56ADw,scheme=https",
            "clientFormFactor": "UNKNOWN_FORM_FACTOR",
            "countryLocationInfo": {
                "countryCode": "GB",
                "countrySource": "COUNTRY_SOURCE_IPGEO_INDEX"
            },
            "browserName": "Firefox",
            "browserVersion": "85.0",
            "screenWidthPoints": 5733,
            "screenHeightPoints": 1433,
            "screenPixelDensity": 1,
            "screenDensityFloat": 0.6,
            "utcOffsetMinutes": 0,
            "userInterfaceTheme": "USER_INTERFACE_THEME_LIGHT",
            "mainAppWebInfo": {
                "graftUrl": "https://www.youtube.com/playlist?list=UUTZ35GQfSb0RsPdREhWZrtg"
            },
            "timeZone": "Europe/London"
        },
        "user": {
            "gaiaId": "1097453154661",
            "userId": "14077168010",
            "lockedSafetyMode": false
        },
        "request": {
            "useSsl": true,
            "sessionId": "6916434730968166000",
            "parentEventId": {
                "timeUsec": "1610366049003706",
                "serverIp": "99695580",
                "processId": "1175011765"
            },
            "internalExperimentFlags": [],
            "consistencyTokenJars": []
        },
        "clickTracking": {
            "clickTrackingParams": "CDUQ7zsYACITCMnexoLpk-4CFZKx1QodIKIG0w=="
        },
        "clientScreenNonce": "MC41MjAzNjcwOTQxODAwMTU.",
        "adSignalsInfo": {
            "params": [
                {
                    "key": "dt",
                    "value": "1610366049764"
                },
                {
                    "key": "flash",
                    "value": "0"
                },
                {
                    "key": "frm",
                    "value": "0"
                },
                {
                    "key": "u_tz",
                    "value": "0"
                },
                {
                    "key": "u_his",
                    "value": "9"
                },
                {
                    "key": "u_java",
                    "value": "false"
                },
                {
                    "key": "u_h",
                    "value": "2400"
                },
                {
                    "key": "u_w",
                    "value": "5733"
                },
                {
                    "key": "u_ah",
                    "value": "2400"
                },
                {
                    "key": "u_aw",
                    "value": "5733"
                },
                {
                    "key": "u_cd",
                    "value": "24"
                },
                {
                    "key": "u_nplug",
                    "value": "0"
                },
                {
                    "key": "u_nmime",
                    "value": "0"
                },
                {
                    "key": "bc",
                    "value": "31"
                },
                {
                    "key": "bih",
                    "value": "1433"
                },
                {
                    "key": "biw",
                    "value": "5713"
                },
                {
                    "key": "brdim",
                    "value": "0,0,0,0,5733,0,5733,2400,5733,1433"
                },
                {
                    "key": "vis",
                    "value": "1"
                },
                {
                    "key": "wgl",
                    "value": "true"
                },
                {
                    "key": "ca_type",
                    "value": "image"
                }
            ],
            "bid": "ANyPxKp8h8KEEwCqi13W3uXn5bOqCXQ8VwYxMiyOfcV9FkjWVtFh_wjeS2-KgzuPsUjJMo_lMZ02nrzgWTEJQowgNfpRJ5BSww"
        }
    },
    "continuation": token
});
    value.to_string()
}

fn build_playlist_url(playlist: &String) -> String {
    format!("https://www.youtube.com/playlist?list={}", playlist) // TODO format isnt optimal should probably just append
}

pub fn get_raws_from_file() -> Result<PlaylistVideoListRenderer, Error> {
    let rdr = File::open("./ytInitialData.json")?;
    let res: Value = serde_json::from_reader(rdr)?;
    let data = extract_initial_data(res)?;
    Ok(data)
}

fn extract_initial_data(value: Value) -> Result<PlaylistVideoListRenderer, Error> {
    let data: PlaylistVideoListRenderer = value.dot_get("contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.playlistVideoListRenderer")?.expect("Failed to read for renderer");
    Ok(data)
}
fn extract_data(value: Value) -> Result<PlaylistVideoListRenderer, Error> {
    let data: Vec<Content4> = value.dot_get("onResponseReceivedActions.0.appendContinuationItemsAction.continuationItems")?.expect("Failed to read command contents");
    let contents = data.iter().filter(|content| content.playlist_video_renderer.is_some()).cloned().collect();
    Ok(PlaylistVideoListRenderer { contents })
}