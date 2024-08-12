use std::error::Error;

use colour::{dark_magenta, magenta};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let resp = ureq::get("https://newsapi.org/v2/everything?q=tesla&from=2024-07-12&sortBy=publishedAt&apiKey=08274588b6bd4965a5e45162b91dfd51").call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&resp)?;

    Ok(articles)
}

fn render_articles(articles: Articles) {
    for i in &articles.articles{
        dark_magenta!("> {}\n", i.title);
        magenta!("{}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let news_url = "https://newsapi.org/v2/top-headlines?sources=techcrunch&apiKey=";
    let news = get_articles(news_url)?;

    render_articles(news);

    println!("Compiled");

    Ok(())
}
