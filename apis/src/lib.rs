use serde::Deserialize;

/**
Enum representing possible errors that can occur while interacting with the News API.

Variants:
- `RequestFailed`: Indicates that the request to the API failed. It wraps the error from the `ureq` crate.
- `ParsingFailed`: Indicates that the response could not be converted to a string. It wraps the error from the standard IO library.
- `ArticleParsingFailed`: Indicates that the response could not be parsed into articles. It wraps the error from the `serde_json` crate.
*/
#[derive(thiserror::Error, Debug)]
pub enum NewsAPIError {
    /// Represents an error that occurs when the request to the API fails.
    #[error("Failed to get response from API.")]
    RequestFailed(ureq::Error),

    /// Represents an error that occurs when the response could not be converted to text.
    #[error("Failed to convert response to text.")]
    ParsingFailed(std::io::Error),

    /// Represents an error that occurs when parsing the response into articles fails.
    #[error("Failed to parse articles from response.")]
    ArticleParsingFailed(serde_json::Error),
}

/**
Struct representing the structure of the JSON response from the News API.

Fields:
- `articles`: A vector of `Article` structs that hold the details of each article.
*/
#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

/**
Struct representing an individual article within the News API response.

Fields:
- `title`: The title of the article.
- `url`: The URL of the article.
*/
#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

/**
Fetches articles from a given URL and returns them as an `Articles` struct.

This function sends a GET request to the specified URL, attempts to parse the response
into a string, and then deserializes the string into an `Articles` struct.

### Arguments

* `url` - A string slice that holds the URL from which the articles are to be fetched.

### Returns

This function returns a `Result` that, on success, contains an `Articles` struct with the
fetched articles. On failure, it returns a `NewsAPIError` indicating what went wrong.

### Errors

This function can return the following errors:
* `NewsAPIError::RequestFailed` - If the request to the API fails.
* `NewsAPIError::ParsingFailed` - If the response could not be converted to a string.
* `NewsAPIError::ArticleParsingFailed` - If the response could not be parsed into articles.
*/
pub fn get_articles(url: &str) -> Result<Articles, NewsAPIError> {
    // Send a GET request to the provided URL using the `ureq` crate.
    let resp = ureq::get(url)
        .call()
        .map_err(|e| NewsAPIError::RequestFailed(e))? // Map any request errors to `NewsAPIError::RequestFailed`.
        .into_string()
        .map_err(|e| NewsAPIError::ParsingFailed(e))?; // Convert the response to a string, mapping errors to `NewsAPIError::ParsingFailed`.

    // Deserialize the JSON string into an `Articles` struct, mapping any errors to `NewsAPIError::ArticleParsingFailed`.
    let articles: Articles =
        serde_json::from_str(&resp).map_err(|e| NewsAPIError::ArticleParsingFailed(e))?;

    // Return the deserialized articles.
    Ok(articles)
}
