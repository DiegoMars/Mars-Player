use dotenvy::from_path;
use rspotify::{model::Market, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use serde_json::json;
use std::{fs, io, path::PathBuf};

#[tokio::main]
async fn get_spotify() -> AuthCodeSpotify {
    let env_path = PathBuf::from("../../Mars-Player-dl-Logic/.env");
    from_path(env_path).ok();

    let creds = Credentials::from_env().unwrap(); // Grabs ID and secret

    let scopes = scopes!(
        "user-library-read",
        "playlist-read-private" // Not using this here
    );
    let oauth = OAuth::from_env(scopes).unwrap(); // Applys scopes and looks for redirect url

    let path: PathBuf = ["..", "..", "Mars-Player-dl-Logic", "token.json"]
        .iter()
        .collect();
    fs::create_dir_all(&path).unwrap(); // Creates director if it doesn't exist

    let config = rspotify::Config {
        token_cached: true,
        token_refreshing: true,
        cache_path: path,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(false).unwrap(); // Grabs redirect url
    spotify.prompt_for_token(&url).unwrap(); // Starts authentication, should hanle the token
                                             // stuff too.

    return spotify;
}

#[tokio::main]
#[tauri::command]
pub async fn pull_songs() {
    let spotify: AuthCodeSpotify = get_spotify();
    let liked_songs = spotify
        .current_user_saved_tracks_manual(Some(Market::FromToken), Some(10), Some(0))
        .unwrap();

    let json_data = json!(liked_songs);

    let mut path: PathBuf = ["..", "data"].iter().collect();
    fs::create_dir_all(&path).unwrap();
    path.push("liked_songs.json");

    let file = fs::File::create(&path).unwrap();
    let writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &json_data).expect_err("Failed to write");
}
