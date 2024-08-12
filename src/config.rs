use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents the configuration structure containing an API key.
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    api_key: String,
}

/**
Returns the path to the configuration file.

The configuration file is located at:

- Linux: `$XDG_CONFIG_HOME/pheme/config.toml` (or `$HOME/.config/pheme/config.toml` if `$XDG_CONFIG_HOME` is not set)
- macOS: `$HOME/Library/Application Support/pheme/config.toml`
- Windows: `{FOLDERID_RoamingAppData}\pheme\config.toml`

The directory is created if it doesn't exist.

# Returns

A `PathBuf` representing the path to the configuration file.
*/
fn get_config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("Failed to get user's config directory");
    config_dir.push("pheme");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir.push("config.toml");
    config_dir
}

/**
Reads the API key from the configuration file.

Attempts to read the configuration file and deserialize it to retrieve the API key.

# Returns

An `Option<String>`, which is `Some(api_key)` if the API key is successfully read,
or `None` if the configuration file does not exist or cannot be read.
*/
fn read_api_key() -> Option<String> {
    let config_path = get_config_path();
    if let Ok(config_str) = fs::read_to_string(&config_path) {
        toml::from_str::<Config>(&config_str)
            .ok()
            .map(|config| config.api_key)
    } else {
        None
    }
}

/**
Writes the given API key to the configuration file.

Saves the provided API key into the configuration file. If the file or directory
does not exist, they will be created.

# Arguments

* `api_key` - The API key to be written to the configuration file.
*/
fn write_api_key(api_key: String) {
    let config_path = get_config_path();
    let config = Config {
        api_key: api_key.clone(),
    };
    fs::write(&config_path, toml::to_string_pretty(&config).unwrap()).unwrap();
}

/**
Prompts the user to enter an API key.

Displays a message asking the user to enter their API key and reads the input from the command line.

# Returns

The API key entered by the user as a `String`.
*/
fn ask_api_key() -> String {
    println!("Please enter your NEWSAPI key (https://newsapi.org/register): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/**
Loads the API key from the configuration file or prompts the user if it is not present.

Tries to read the API key from the configuration file. If the file does not exist or the key cannot be read,
the user is prompted to enter the API key, which is then saved to the configuration file.

# Returns

The API key as a `String`. If the key is read from the file, it is returned directly. If the user is prompted,
the entered key is saved and returned.
*/
pub fn load_api_key() -> String {
    if let Some(api_key) = read_api_key() {
        api_key
    } else {
        // Config file doesn't exist or has an error, prompt the user
        let api_key = ask_api_key();
        write_api_key(api_key.clone());
        api_key
    }
}
