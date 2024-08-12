use crate::args::Args;
use crate::config::load_api_key;
use std::error::Error;
use urlencoding::encode;

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
    let news_api_key = load_api_key();

    Ok(format!("{}?apiKey={}", url, news_api_key))
}

/**
Builds the query parameter string for the News API request.

This function takes the command-line arguments (`Args`) and constructs a URL-encoded 
query parameter string based on the provided options.

### Arguments:

* `args` - A reference to the `Args` struct containing the command-line arguments.

### Returns:

* A `String` containing the formatted query parameters for the News API request. 
*/
pub fn build_params(args: &Args) -> String {
    let mut params: String = "".to_string();

    // Add optional parameters to the URL.
    if let Some(q) = &args.q {
        params.push_str(&format!("&q={}", encode(q)));
    }
    if let Some(search_in) = &args.search_in {
        params.push_str(&format!("&searchIn={}", search_in));
    }
    if let Some(sources) = &args.sources {
        params.push_str(&format!("&sources={}", sources));
    }
    if let Some(domains) = &args.domains {
        params.push_str(&format!("&domains={}", domains));
    }
    if let Some(exclude_domains) = &args.exclude_domains {
        params.push_str(&format!("&excludeDomains={}", exclude_domains));
    }
    if let Some(language) = &args.language {
        params.push_str(&format!("&language={}", language));
    }
    if let Some(sort_by) = &args.sort_by {
        params.push_str(&format!("&sortBy={}", sort_by));
    }
    if let Some(number) = &args.number {
        params.push_str(&format!("&pageSize={}", number));
    }
    if let Some(page) = &args.page {
        params.push_str(&format!("&page={}", page));
    }

    params
}
