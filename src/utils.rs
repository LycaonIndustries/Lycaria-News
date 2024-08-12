use dotenv::dotenv;
use std::error::Error;

const NEWS_BASE_URL: &str = "https://newsapi.org/v2";

/**
Generates the complete URL for fetching news articles from the News API.

This function takes a string of query parameters and constructs the full API URL,
including the base URL, endpoint, and API key.

If no parameters are provided, it defaults to fetching top headlines with "rust" as the query.

### Arguments:

* `params` - A string containing URL encoded query parameters for the News API request.

### Returns:

* A `String` representing the complete URL for the News API request.
*/
pub fn generate_url(params: &str) -> String {
    let mut url = if params.is_empty() {
        format!("{}/top-headlines", NEWS_BASE_URL)
    } else {
        format!("{}/everything", NEWS_BASE_URL)
    };

    // Sign the URL
    url = sign_api_key(&url).unwrap_or_else(|err| {
        eprintln!("Error signing API key: {}", err);
        std::process::exit(1);
    });

    // Append params at the end
    if !params.is_empty() {
        url.push_str(params);
        return url;
    }

    url.push_str("&q=rust");

    url
}

/**
Signs the API request with the API key from the environment.

This function takes a base URL and appends the `apiKey` parameter to it,
retrieving the key from the `NEWS_API_KEY` environment variable.

### Arguments:

* `url` - The base URL to sign.

### Returns:

* A `Result<String, Box<dyn Error>>` containing the signed URL if successful,
  or a `Box<dyn Error>` if the API key cannot be retrieved.
*/
fn sign_api_key(url: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let news_api_key = std::env::var("NEWS_API_KEY")
        .map_err(|_| "Environment variable NEWS_API_KEY must be set".to_string())?;

    Ok(format!("{}?apiKey={}", url, news_api_key))
}
