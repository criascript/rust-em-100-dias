
use scraper::{Html, Selector};
use reqwest::blocking::get;

fn main() {
    
    let username = "criascript";
    let url: String = format!("https://www.tiktok.com/@{}?lang=pt-BR", username);

    let response = reqwest::blocking::get(
        url,
    )
    .unwrap()
    .text()
    .unwrap();

    let document = Html::parse_document(&response);

    let binding_following_count = Selector::parse("strong[data-e2e=\"following-count\"]").unwrap();
    let following_count = document.select(&binding_following_count);

    let binding_followers_count = Selector::parse("strong[data-e2e=\"followers-count\"]").unwrap();
    let followers_count = document.select(&binding_followers_count);

    let binding_likes_count = Selector::parse("strong[data-e2e=\"likes-count\"]").unwrap();
    let likes_count = document.select(&binding_likes_count);

    for element in following_count {
        println!("{} está seguindo {} pessoas", username, element.text().collect::<Vec<_>>().join(""));
    }

    for element in followers_count {
        println!("{} tem {} seguidores", username, element.text().collect::<Vec<_>>().join(""));
    }

    for element in likes_count {
        println!("{} tem {} curtidas", username, element.text().collect::<Vec<_>>().join(""));
    }

}
