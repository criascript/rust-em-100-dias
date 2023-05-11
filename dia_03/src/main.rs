#![allow(unused_imports)]
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};

struct UserProfile {
    following_count: f64,
    followers_count: f64,
    likes_count: f64,
}

fn parse_count(data: &str) -> f64 {
    let re_k = Regex::new(r#"^\d+(\.\d+)?[Kk]$"#).unwrap();
    let re_m = Regex::new(r#"^\d+(\.\d+)?[Mm]$"#).unwrap();

    if re_k.is_match(data) {
        let num = data.trim_end_matches(|c| c == 'K' || c == 'k');
        return num.parse::<f64>().unwrap_or(0.0) * 1000.0;
    }

    if re_m.is_match(data) {
        let num = data.trim_end_matches(|c| c == 'M' || c == 'm');
        return num.parse::<f64>().unwrap_or(0.0) * 1000000.0;
    }

    data.parse::<f64>().unwrap_or(0.0)
}

fn main() {
    let username = "criascript";
    let url: String = format!("https://www.tiktok.com/@{}?lang=pt-BR", username);

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&response);

    let binding_following_count = Selector::parse("strong[data-e2e=\"following-count\"]").unwrap();
    let binding_followers_count = Selector::parse("strong[data-e2e=\"followers-count\"]").unwrap();
    let binding_likes_count = Selector::parse("strong[data-e2e=\"likes-count\"]").unwrap();

    let mut following_count = document.select(&binding_following_count);
    let mut followers_count = document.select(&binding_followers_count);
    let mut likes_count = document.select(&binding_likes_count);

    let following_count_str = following_count.next().unwrap().text().collect::<String>();
    let followers_count_str = followers_count.next().unwrap().text().collect::<String>();
    let likes_count_str = likes_count.next().unwrap().text().collect::<String>();

    let profile = UserProfile {
        following_count: parse_count(&following_count_str),
        followers_count: parse_count(&followers_count_str),
        likes_count: parse_count(&likes_count_str),
    };

    println!("{}: following_count={}, followers_count={}, likes_count={}", username, profile.following_count, profile.followers_count, profile.likes_count);


}
