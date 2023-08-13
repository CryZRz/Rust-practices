use gloo_net::http::Request;
use serde::Deserialize;
use crate::env::YOUTUBE_KEY;

pub async fn search_youtube(text_to_search: String) -> Result<VideoItem, gloo_net::Error>{
    //llamar api de youtube
    let query_url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}", 
        text_to_search    
        );
    
    let mut auth = String::from("Bearer ");
    auth.push_str(YOUTUBE_KEY);
    
    let response = Request::get(&query_url)
    .header("Authorization", &auth)
    .send().await?;
    
    let search_result = response.json::<SearchResult>().await?;
    let video = search_result.items.first();

    let empty_video = build_empty_video();
    let video = match video {
        Some(video) => video,
        None => &empty_video
    };

    Ok(video.clone())
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult{
    region_code: String,
    items: Vec<VideoItem>
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String
}

fn build_empty_video() -> VideoItem{
    VideoItem { 
        id: VideoItemId { kind: "".to_string(), video_id: "".to_string() }, 
        snippet: VideoSnippet { title: "".to_string(), description: "".to_string() } }
}