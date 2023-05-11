#![allow(unused_imports)]
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use serde_json::json;


fn parse_count(data: &str) -> f64 {
    let re_k: Regex = Regex::new(r#"^\d+(\.\d+)?[Kk]$"#).unwrap();
    let re_m: Regex = Regex::new(r#"^\d+(\.\d+)?[Mm]$"#).unwrap();

    if re_k.is_match(data) {
        let num: &str = data.trim_end_matches(|c| c == 'K' || c == 'k');
        return num.parse::<f64>().unwrap_or(0.0) * 1000.0;
    }

    if re_m.is_match(data) {
        let num: &str = data.trim_end_matches(|c| c == 'M' || c == 'm');
        return num.parse::<f64>().unwrap_or(0.0) * 1000000.0;
    }

    data.parse::<f64>().unwrap_or(0.0)
}

fn main() {
    let username: &str = "criascript";
    let url: String = format!("https://www.tiktok.com/@{}?lang=pt-BR", username);
    let response: String = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document: Html = Html::parse_document(&response);

    let binding_following_count: Selector = Selector::parse("strong[data-e2e=\"following-count\"]").unwrap();
    let binding_followers_count: Selector = Selector::parse("strong[data-e2e=\"followers-count\"]").unwrap();
    let binding_likes_count: Selector = Selector::parse("strong[data-e2e=\"likes-count\"]").unwrap();
    let binding_views_count = Selector::parse("strong.video-count[data-e2e=video-views]").unwrap();
    let binding_user_title_posts = Selector::parse("[data-e2e=\"user-post-item-desc\"]").unwrap();
    
    let views: Vec<scraper::element_ref::ElementRef> = document.select(&binding_views_count).collect();
    let views_values: Vec<i32> = views.iter().map(|view| {
        let view_str: String = view.text().collect::<String>();
        parse_count(&view_str).round() as i32
    }).collect();
    let views_sum: i32 = views_values.iter().sum();
    
    let posts: Vec<scraper::element_ref::ElementRef> = document.select(&binding_user_title_posts).collect();
    
    let title_views: Vec<(String, i32)> = posts.iter().zip(views_values.iter()).map(|(post, views)| {
        let binding_title: Selector = Selector::parse("a").unwrap();
        let title: scraper::ElementRef = post.select(&binding_title).next().unwrap();
        let title_attr: &str = title.value().attr("title").unwrap();
        (title_attr.to_string(), *views)
    }).collect();
    
    let title_views_map: std::collections::HashMap<String, i32> = title_views.into_iter().collect();
    
    let mut following_count: scraper::html::Select = document.select(&binding_following_count);
    let mut followers_count: scraper::html::Select = document.select(&binding_followers_count);
    let mut likes_count: scraper::html::Select = document.select(&binding_likes_count);

    let following_count_str: String = following_count.next().unwrap().text().collect::<String>();
    let followers_count_str: String = followers_count.next().unwrap().text().collect::<String>();
    let likes_count_str: String = likes_count.next().unwrap().text().collect::<String>();

    let following: i32 = parse_count(&following_count_str).round() as i32;
    let followers: i32 = parse_count(&followers_count_str).round() as i32;
    let likes: i32 = parse_count(&likes_count_str).round() as i32;

    //create a hashmap with the data
    let mut data: std::collections::HashMap<&str, String> = std::collections::HashMap::new();

    data.insert("following_count", following.to_string());
    data.insert("followers_count", followers.to_string());
    data.insert("likes_count", likes.to_string());
    data.insert("total_views_count", views_sum.to_string());
    data.insert("videos", serde_json::to_string(&title_views_map).unwrap());

    println!("{}", serde_json::to_string_pretty(&data).unwrap());

}
