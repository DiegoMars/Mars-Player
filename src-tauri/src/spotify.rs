// When defining commands in a separate module they should be marked as pub.

// AuthCodeSpotify is the client you'll use for user-authenticated actions.
// Credentials loads your Spotify API key and secret.
// OAuth defines the scope (what permissions you’re asking for).
// scopes is a helper macro for setting those permissions.
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

// loads key-value pairs from a .env file into the environment
use dotenvy::from_path;

// Brings in Rust’s built-in environment handling (env::var) and PathBuf for safe file paths.
use std::{env, path::PathBuf};

#[tokio::main]
#[tauri::command]
pub async fn pull_songs() {
    let env_path = std::path::PathBuf::from("../../../Mars-Player-dl-Logic/.env");
    dotenvy::from_path(env_path).ok();

    let creds = Credentials::from_env().unwrap(); // Grabs ID and secret

    let scopes = scopes!(
        "user-library-read"
        // "playlist-read-private", // Not using this here
    );
    let oauth = OAuth::from_env(scopes).unwrap(); // Applys scopes and looks for redirect url

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap(); // Grabs redirect url
    spotify.prompt_for_token(&url).unwrap(); // Starts authentication
}
