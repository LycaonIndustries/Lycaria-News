// Importing necessary modules and functions.
mod args;
mod utils;
use clap::Parser;

use apis::{get_articles, render_articles, Articles};
use std::error::Error;

/**
Main function to fetch and display headlines from the News API.

This function loads the API key, constructs the API URL with optional parameters,
fetches the articles, and then renders them.
*/
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments.
    let args = args::Args::parse();

    let params = utils::build_params(&args);

    let url = utils::generate_url(&params);

    // Fetch the articles from the News API.
    let news: Articles = get_articles(&url)?;

    // Render the fetched articles to the console.
    render_articles(news);

    Ok(())
}
