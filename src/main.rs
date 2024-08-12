// Importing necessary modules and functions.
// `get_articles` and `Articles` are used to fetch and represent the articles from the API.
// `dark_blue_ln`, `dark_magenta`, and `magenta` are used for colored terminal output.
// `dotenv` is used to load environment variables from a `.env` file.
// `Error` trait is used for error handling.
use apis::{get_articles, Articles};
use colour::{dark_blue_ln, dark_magenta};
use dotenv::dotenv;
use std::error::Error;

/// Renders the articles to the console with colored output.
///
/// This function takes an `Articles` struct and prints each article's title and URL to the console.
/// The title is printed in dark magenta, and the URL is printed in dark blue.
///
/// # Arguments
///
/// * `articles` - An `Articles` struct containing the list of articles to render.
pub fn render_articles(articles: Articles) {
    for i in &articles.articles {
        // Print the article's title in dark magenta.
        dark_magenta!("> {}\n", i.title);

        // Print the article's URL in dark blue, followed by two newlines for spacing.
        dark_blue_ln!("{}\n\n", i.url);
    }
}

/// Main function to fetch and display top headlines from the News API.
///
/// This function loads the API key from environment variables, constructs the API URL,
/// fetches the articles using the `get_articles` function, and then renders them using `render_articles`.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` on success, or an error if any step fails.
///
/// # Errors
///
/// This function can return the following errors:
/// * If the `.env` file cannot be loaded.
/// * If the `NEWS_API_KEY` environment variable is not set.
/// * If the request to the News API fails or if the articles cannot be fetched or parsed.
fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from a `.env` file.
    dotenv()?;

    // Retrieve the API key from the environment variable `NEWS_API_KEY`.
    let news_api_key = std::env::var("NEWS_API_KEY")?;

    // Base URL for fetching top headlines from the News API.
    let url = "https://newsapi.org/v2/top-headlines?sources=techcrunch&apiKey=";

    // Combine the base URL with the API key to create the full request URL.
    let news_url = format!("{}{}", url, news_api_key);

    // Fetch the articles from the News API.
    let news: Articles = get_articles(&news_url)?;

    // Render the fetched articles to the console.
    render_articles(news);

    // Return `Ok(())` to indicate that the function executed successfully.
    Ok(())
}
