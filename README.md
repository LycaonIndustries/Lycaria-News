# Pheme

A Rust-based command-line news aggregator that fetches and displays news articles from the News API.  

## Features

- **Customizable News Queries:** Fetch news based on keywords, sources, languages, and more using various command-line arguments.
- **Colored Output:**  Displays news headlines and URLs with colored formatting for readability.
- **Environment Variable Management:** Securely load your News API key from a `.env` file.
- **Robust Error Handling:** Handles API request errors, parsing errors, and missing environment variables gracefully.
- **Modular Codebase:** Organized into modules (`api`, `args`, `render`, `utils`) for maintainability.

## Installation

### Prerequisites

- **Rust:** Install the Rust toolchain from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **News API Key:** Obtain a free API key from [https://newsapi.org/register](https://newsapi.org/register)

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/news-aggregator-cli.git
   cd news-aggregator-cli
   ```

2. Set up the `.env` file:

   Create a `.env` file in the root directory of the project and add your News API key:

   ```bash
   echo "NEWS_API_KEY=your_api_key_here" > .env
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. Run the application:

   ```bash
   cargo run --release
   ```

## Usage

Run cargo run --release -- -h (or ./target/release/pheme -h after building) to see the available command-line options:

```md
USAGE:
    pheme [OPTIONS]

OPTIONS:
   -d, --domains <domains>              Comma-separated string of domains to restrict the search to
   -e, --exclude-domains <exclude-domains>
                                       Comma-separated string of domains to remove from the
                                       results
   -l, --language <language>          The 2-letter ISO-639-1 code of the language you want
                                       headlines for
   -n, --number <number>              The number of results to return per page (max 100)
   -p, --page <page>                    Use this to page through the results
   -q, --q <q>                        Keywords or phrases to search for in the article title and
                                       body
   -s, --sources <sources>              Comma-separated string of identifiers for news sources or
                                       blogs
   --search-in <search-in>            The fields to restrict your q search to. Possible options:
                                       title, description, content (comma-separated)
   --sort-by <sort-by>                The order to sort the articles in. Possible options:
                                       relevancy, popularity, publishedAt
   -h, --help                        Print help information
   -V, --version                     Print version information
```

### Examples

- Search for news about artificial intelligence:

```bash
cargo run --release -- -q "artificial intelligence"
```

- Get news from specific sources:

```bash
cargo run --release -- -s "bbc-news,the-verge"
```

- Filter by language and sort by relevance

```bash
cargo run --release -- -q "technology" --language en --sort-by relevancy
```

## Configuration

### Environment Variables

- **`NEWS_API_KEY`**: Your API key for accessing the News API (required).
  
### Customizing the News Source

If you want to fetch news from a different source, you can modify the `NEWS_DEFAULT_SOURCE` constant in the code:

```rust
const NEWS_DEFAULT_SOURCE: &str = "your_preferred_source";
```

Alternatively, you can extend the project to accept a command-line argument for the news source.

## Error Handling

The application includes error handling for the following scenarios:

- **API Request Failures**: If the request to the News API fails.
- **Parsing Failures**: If the response from the API cannot be parsed into the expected format.
- **Missing API Key**: If the `NEWS_API_KEY` environment variable is not set or found.

## Dependencies

This project relies on the following crates:

- `ureq`: For making HTTP requests to the News API.
- `serde` and `serde_json`: For JSON parsing.
- `dotenv`: For loading environment variables.
- `colour`: For colored terminal output.
- `thiserror`: For defining custom error types.
- `clap`: For parsing command-line arguments.
