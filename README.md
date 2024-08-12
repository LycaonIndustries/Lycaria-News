# Pheme

This Rust-based command-line interface (CLI) application fetches and displays top headlines from a specified news source using the News API. The application leverages environment variables for secure API key management and provides a simple, colored output of the news articles in the terminal.

## Features

- **Fetch News Headlines**: Retrieves top headlines from a configurable news source using the News API.
- **Colored Output**: Displays article titles and URLs in distinct colors for enhanced readability.
- **Environment Variable Management**: Utilizes the `dotenv` crate to load API keys securely from a `.env` file.
- **Error Handling**: Implements robust error handling to manage API request failures, parsing issues, and missing environment variables.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have the Rust toolchain installed)
- A [News API key](https://newsapi.org/register) (You'll need to sign up to get a free API key)

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

Upon running the application, it will fetch the latest top headlines from the configured news source (default is TechCrunch) and display them in the terminal with colored formatting:

- **Title**: Displayed in bold magenta
- **URL**: Displayed in dark blue

### Example Output

```text
->> Tech Giants Announce New Collaboration
https://www.example.com/article1

->> Breakthrough in AI Technology
https://www.example.com/article2
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
