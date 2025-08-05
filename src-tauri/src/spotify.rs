use dotenvy::from_path;
use rspotify::{
    model::{Market, SavedTrack},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use serde_json::json;
use std::{fs, io, path::PathBuf};
use tauri::Emitter;

async fn get_spotify() -> AuthCodeSpotify {
    let env_path = PathBuf::from("../../Mars-Player-dl-Logic/.env");
    from_path(env_path).ok();

    let creds = Credentials::from_env().unwrap(); // Grabs ID and secret

    let scopes = scopes!(
        "user-library-read",
        "playlist-read-private" // Not using this here
    );
    let oauth = OAuth::from_env(scopes).unwrap(); // Applys scopes and looks for redirect url

    let mut path = PathBuf::from("../../Mars-Player-dl-Logic/");
    fs::create_dir_all(&path).unwrap(); // Creates director if it doesn't exist
    path.push("token.json");

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
pub async fn pull_songs(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let spotify = get_spotify().await;

        let mut liked_songs: Vec<SavedTrack> = Vec::new();
        let mut offset = 0;
        let limit = 50;
        let mut page_count = 0;

        loop {
            match spotify.current_user_saved_tracks_manual(
                Some(Market::FromToken),
                Some(limit),
                Some(offset),
            ) {
                Ok(page) => {
                    liked_songs.extend(page.items);
                    offset += limit;
                    page_count += 1;

                    // Emit progress to frontend
                    let _ = app.emit("fetch-progress", page_count * limit);

                    if page.next.is_none() {
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }

        let json_data = json!(liked_songs);

        // let mut path = PathBuf::from("../data/");
        let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/");
        fs::create_dir_all(&path).unwrap();
        path.push("liked_songs.json");

        // Writting stuff
        let file = fs::File::create(&path).unwrap();
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &json_data).expect("Written to json file!");

        // Emit completion event
        let _ = app.emit("fetch-complete", liked_songs.len());
    });
}
