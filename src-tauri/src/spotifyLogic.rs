// When defining commands in a separate module they should be marked as pub.

// AuthCodeSpotify is the client you'll use for user-authenticated actions.
// Credentials loads your Spotify API key and secret.
// OAuth defines the scope (what permissions you’re asking for).
// scopes is a helper macro for setting those permissions.
use rspotify::{clients::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth};

// loads key-value pairs from a .env file into the environment
use dotenvy::from_path;

// Brings in Rust’s built-in environment handling (env::var) and PathBuf for safe file paths.
use std::{env, path::PathBuf};

// Using to open the authentication url
use open

#[tokio::main]
pub async fn pull_songs() {
    let env_path = PathBuf::from("../../../../Mars-Player-dl-Logic/.env"); // Makes path to env file
    env_path.display();

    from_path(env_path).expect("Failed to load .env"); // Loads variables

    let creds = Credentials::from_env().expect("Missing credentials");
    // Looks for the correct values, not adding all the values from the .env file

    let oauth = OAuth {
        scopes: scopes!("user-library-read playlist-read-private"),
        redirect_uri: env::var("RSPOTIFY_REDIRECT_URI").unwrap(),
        ..Default::default() // fills in the other OAuth fields with their default values.
    };

    let mut spotify = AuthCodeSpotify::new(creds, oauth); // Makes spotify client object
    let url = spotify.get_authorize_url(false).unwrap(); // Generate authentication url

    // Stopping to learn rust
}
