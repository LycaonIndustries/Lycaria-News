// Importing necessary modules and functions.
mod args;
mod utils;
use clap::Parser;

use apis::{get_articles, render_articles, Articles};
use std::error::Error;
use urlencoding::encode;

/**
Main function to fetch and display headlines from the News API.

This function loads the API key, constructs the API URL with optional parameters,
fetches the articles, and then renders them.
*/
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments.
    let args = args::Args::parse();

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

    let url = utils::generate_url(&params);

    // Fetch the articles from the News API.
    let news: Articles = get_articles(&url)?;

    // Render the fetched articles to the console.
    render_articles(news);

    Ok(())
}
