pub mod traits;
pub mod types;

use std::{
    collections::{BTreeMap, HashSet},
    io::Write,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use skyscraper::{html, xpath};
use traits::{GetFirstNode, ParseJson};
use types::{Challenge, UserData, VideoData};

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
    hashtags_used: Vec<String>,
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
    cover: String,
    challenges: Vec<Challenge>,
}

//todo: use hashtag search to get more data
// https://www.tiktok.com/tag/hashtagname?lang=pt-BR

fn get_videos(item_module: BTreeMap<String, VideoData>) -> Option<BTreeMap<String, Video>> {
    let mut videos = BTreeMap::new();
    for (id, video) in item_module {
        videos.insert(
            id,
            Video {
                desc: video.desc,
                play_count: video.stats.play_count,
                heart_count: video.stats.digg_count,
                comment_count: video.stats.comment_count,
                create_time: video.create_time,
                cover: video.video.cover,
                challenges: video.challenges,
            },
        );
    }

    Some(videos)
}

fn parse_user_data(mut user_data: UserData) -> Option<TikTokData> {
    let videos = get_videos(user_data.item_module)?;

    let user_stats = user_data.user_module.stats.remove(USERNAME)?;
    let play_average =
        videos.values().map(|v| v.play_count).sum::<i64>() as f64 / videos.len() as f64;
    let play_count = videos.values().map(|v| v.play_count).sum();
    let comment_count = videos.values().map(|v| v.comment_count).sum();

    let hashtags_used = videos
        .values()
        .flat_map(|v| v.challenges.clone())
        .map(|c| c.title)
        .collect::<Vec<_>>();

    let unique_hashtags: HashSet<String> = hashtags_used.into_iter().collect();
    let mut hashtags_used: Vec<String> = unique_hashtags.into_iter().collect();
    hashtags_used.sort();

    // get_hashtag_data(hashtags_used);

    Some(TikTokData {
        following_count: user_stats.following_count,
        follower_count: user_stats.follower_count,
        like_count: user_stats.heart_count,
        play_count,
        play_average,
        comment_count,
        videos,
        hashtags_used,
    })
}

// fn parse_hashtag_data(mut user_data: UserData) -> Option<TikTokData> {
//     None
// }

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

fn get_hashtag_data(data: Vec<String>) -> Result<(), TikTokError> {
    let mut hashtags_data: Vec<_> = Vec::new();

    for hashtag in data {
        let url: String = format!("https://www.tiktok.com/tag/{hashtag}?lang=pt-BR");
        let response: String = reqwest::blocking::get(url)?.text()?;
        let doc = html::parse(&response)?;
        hashtags_data.push(
            xpath::parse(r#"//script[@id="SIGI_STATE"]"#)?
                .get_first_text(&doc)
                .ok_or_else(|| TikTokError::Custom("no data found".to_string()))?
                .parse_json()?,
        );

        std::fs::File::create("raw_hashtags.json")?
            .write_all(serde_json::to_string_pretty(&hashtags_data)?.as_bytes())?;
    }

    Ok(())
}

fn main() {
    get_tiktok_data().unwrap();
}
