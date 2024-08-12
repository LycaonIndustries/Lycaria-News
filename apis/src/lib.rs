use std::error::Error;

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsAPIError {
    #[error("Failed to get response from API.")]
    RequestFailed(ureq::Error),
    #[error("Failed to convert response to text.")]
    ParsingFailed(std::io::Error),
    #[error("Failed to parse articles from response.")]
    ArticleParsingFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsAPIError> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| NewsAPIError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsAPIError::ParsingFailed(e))?;

    let articles: Articles =
        serde_json::from_str(&resp).map_err(|e| NewsAPIError::ArticleParsingFailed(e))?;

    Ok(articles)
}
