// Importing necessary modules and functions.
use apis::{get_articles, Articles};
use clap::Parser;
use colour::{dark_blue_ln, dark_magenta_bold};
use dotenv::dotenv;
use std::error::Error;

const NEWS_BASE_ENDPOINT: &str = "https://newsapi.org/v2/everything";

// Define command-line arguments using clap.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Keywords or phrases to search for in the article title and body.
    #[arg(short, long)]
    q: Option<String>,

    /// The fields to restrict your q search to.
    /// Possible options: title, description, content (comma-separated)
    #[arg(long)]
    search_in: Option<String>,

    /// Comma-separated string of identifiers for news sources or blogs
    #[arg(long)]
    sources: Option<String>,

    /// Comma-separated string of domains to restrict the search to
    #[arg(long)]
    domains: Option<String>,

    /// Comma-separated string of domains to remove from the results
    #[arg(long)]
    exclude_domains: Option<String>,

    /// The 2-letter ISO-639-1 code of the language you want headlines for
    #[arg(short, long)]
    language: Option<String>,

    /// The order to sort the articles in
    /// Possible options: relevancy, popularity, publishedAt
    #[arg(short, long)]
    sort_by: Option<String>,

    /// The number of results to return per page (max 100)
    #[arg(short, long)]
    number: Option<u32>,

    /// Use this to page through the results
    #[arg(short, long)]
    page: Option<u32>,
}

/**
Renders the articles to the console with colored output.

This function takes an `Articles` struct and prints each article's title and URL to the console.
The title is printed in dark magenta, and the URL is printed in dark blue.

### Arguments

* `articles` - An `Articles` struct containing the list of articles to render.
*/
pub fn render_articles(articles: Articles) {
    for i in &articles.articles {
        // Print the article's title in dark magenta bold.
        dark_magenta_bold!("->> {}\n", i.title);

        // Print the article's URL in dark blue, followed by two newlines for spacing.
        dark_blue_ln!("{}\n\n", i.url);
    }
}

/**
Main function to fetch and display headlines from the News API.

This function loads the API key, constructs the API URL with optional parameters,
fetches the articles, and then renders them.
*/
fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from a `.env` file.
    dotenv()?;

    // Parse command-line arguments.
    let args = Args::parse();

    // Retrieve the API key from the environment variable `NEWS_API_KEY`.
    let news_api_key = std::env::var("NEWS_API_KEY")
        .map_err(|_| "Environment variable NEWS_API_KEY must be set")?;

    // Build the API URL with query parameters.
    let mut url = format!("{}?apiKey={}", NEWS_BASE_ENDPOINT, news_api_key);

    // Add optional parameters to the URL.
    if let Some(q) = &args.q {
        url.push_str(&format!("&q={}", q));
    }
    if let Some(search_in) = &args.search_in {
        url.push_str(&format!("&searchIn={}", search_in));
    }
    if let Some(sources) = &args.sources {
        url.push_str(&format!("&sources={}", sources));
    }
    if let Some(domains) = &args.domains {
        url.push_str(&format!("&domains={}", domains));
    }
    if let Some(exclude_domains) = &args.exclude_domains {
        url.push_str(&format!("&excludeDomains={}", exclude_domains));
    }
    if let Some(language) = &args.language {
        url.push_str(&format!("&language={}", language));
    }
    if let Some(sort_by) = &args.sort_by {
        url.push_str(&format!("&sortBy={}", sort_by));
    }
    if let Some(number) = &args.number {
        url.push_str(&format!("&pageSize={}", number));
    }
    if let Some(page) = &args.page {
        url.push_str(&format!("&page={}", page));
    }

    // Fetch the articles from the News API.
    let news: Articles = get_articles(&url)?;

    // Render the fetched articles to the console.
    render_articles(news);

    Ok(())
}
