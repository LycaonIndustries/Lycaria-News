use clap::Parser;

// Define command-line arguments using clap.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Keywords or phrases to search for in the article title and body.
    #[arg(short, long)]
    pub q: Option<String>,

    /// The fields to restrict your q search to.
    /// Possible options: title, description, content (comma-separated)
    #[arg(long)]
    pub search_in: Option<String>,

    /// Comma-separated string of identifiers for news sources or blogs
    #[arg(long)]
    pub sources: Option<String>,

    /// Comma-separated string of domains to restrict the search to
    #[arg(long)]
    pub domains: Option<String>,

    /// Comma-separated string of domains to remove from the results
    #[arg(long)]
    pub exclude_domains: Option<String>,

    /// The 2-letter ISO-639-1 code of the language you want headlines for
    #[arg(short, long)]
    pub language: Option<String>,

    /// The order to sort the articles in
    /// Possible options: relevancy, popularity, publishedAt
    #[arg(short, long)]
    pub sort_by: Option<String>,

    /// The number of results to return per page (max 100)
    #[arg(short, long)]
    pub number: Option<u32>,

    /// Use this to page through the results
    #[arg(short, long)]
    pub page: Option<u32>,
}
