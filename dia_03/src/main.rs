pub mod traits;

use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

use chrono::{DateTime, TimeZone};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use skyscraper::{html, xpath};
use traits::{GetFirstNode, ParseJson};

const USERNAME: &str = "criascript";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TikTokData {
    following_count: i64,
    follower_count: i64,
    like_count: i64,
    play_count: i64,
    play_average: f64,
    comment_count: i64,
    videos: BTreeMap<String, Video>,
}

#[derive(Debug, thiserror::Error)]
enum TikTokError {
    #[error("html parse error: {0}")]
    HtmlParse(#[from] html::parse::ParseError),

    #[error("xpath parse error: {0}")]
    XpathParse(#[from] xpath::parse::ParseError),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("apply error: {0}")]
    Apply(#[from] xpath::ApplyError),

    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("std io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("custom error: {0}")]
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Video {
    desc: String,
    play_count: i64,
    heart_count: i64,
    comment_count: i64,
    create_time: DateTime<chrono::Utc>,
}

type UserData = HashMap<String, HashMap<String, Value>>;

fn get_videos(user_data: &UserData) -> Option<BTreeMap<String, Video>> {
    let mut videos = BTreeMap::new();
    for (id, video) in user_data.get("ItemModule")? {
        let video = video.as_object()?;
        let stats = video.get("stats")?;
        videos.insert(
            id.to_string(),
            Video {
                desc: video.get("desc")?.as_str()?.to_string(),
                play_count: stats.get("playCount")?.as_i64()?,
                heart_count: stats.get("diggCount")?.as_i64()?,
                comment_count: stats.get("commentCount")?.as_i64()?,
                create_time: chrono::Utc
                    .timestamp_opt(video.get("createTime")?.as_str()?.parse().ok()?, 0)
                    .single()?,
            },
        );
    }

    Some(videos)
}

fn parse_user_data(user_data: UserData) -> Option<TikTokData> {
    let videos = get_videos(&user_data)?;

    let user_stats = user_data
        .get("UserModule")?
        .get("stats")?
        .as_object()?
        .get(USERNAME)?
        .as_object()?;

    Some(TikTokData {
        following_count: user_stats.get("followingCount")?.as_i64()?,
        follower_count: user_stats.get("followerCount")?.as_i64()?,
        like_count: user_stats.get("heartCount")?.as_i64()?,
        play_count: videos.values().map(|v| v.play_count).sum(),
        play_average: videos.values().map(|v| v.play_count).sum::<i64>() as f64
            / videos.len() as f64,
        comment_count: videos.values().map(|v| v.comment_count).sum(),
        videos,
    })
}

fn get_tiktok_data() -> Result<(), TikTokError> {
    let url: String = format!("https://www.tiktok.com/@{USERNAME}?lang=pt-BR");
    let response: String = reqwest::blocking::get(url)?.text()?;
    let doc = html::parse(&response)?;
    let user_data = xpath::parse(r#"//script[@id="SIGI_STATE"]"#)?
        .get_first_text(&doc)
        .ok_or_else(|| TikTokError::Custom("no data found".to_string()))?
        .parse_json()?;

    std::fs::File::create("raw.json")?
        .write_all(serde_json::to_string_pretty(&user_data)?.as_bytes())?;

    let tiktok_data = parse_user_data(user_data)
        .ok_or_else(|| TikTokError::Custom("no data found".to_string()))?;

    std::fs::File::create("data.json")?
        .write_all(serde_json::to_string_pretty(&tiktok_data)?.as_bytes())?;
    Ok(())
}

fn main() {
    get_tiktok_data().unwrap();
}
