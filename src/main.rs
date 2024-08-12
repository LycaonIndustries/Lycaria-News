use apis::{get_articles, Articles};
use colour::{dark_blue_ln, dark_magenta, magenta};
use dotenv::dotenv;
use std::error::Error;
pub fn render_articles(articles: Articles) {
    for i in &articles.articles {
        dark_magenta!("> {}\n", i.title);
        dark_blue_ln!("{}\n\n", i.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let news_api_key = std::env::var("NEWS_API_KEY")?;

    let url = "https://newsapi.org/v2/top-headlines?sources=techcrunch&apiKey=";
    let news_url = format!("{}{}", url, news_api_key);
    let news: Articles = get_articles(&news_url)?;

    render_articles(news);
    Ok(())
}
