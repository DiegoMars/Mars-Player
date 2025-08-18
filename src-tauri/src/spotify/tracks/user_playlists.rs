use crate::spotify::auth::get_spotify;
use chrono::{DateTime, Utc};
use rspotify::{model::playlist::SimplifiedPlaylist, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, io, path::PathBuf};
use tauri::Emitter;

#[derive(Serialize, Deserialize)]
struct PlaylistsExport {
    fetched_at: String,
    playlist_number: u32,
    playlists: Vec<SimplifiedPlaylist>,
}

#[tauri::command]
pub async fn pull_playlists(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let spotify = get_spotify().await;

        let mut playlists: Vec<SimplifiedPlaylist> = Vec::new();
        let mut offset = 0;
        let limit = 50;
        let mut page_count = 0;

        loop {
            match spotify.current_user_playlists_manual(Some(limit), Some(offset)) {
                Ok(page) => {
                    playlists.extend(page.items);
                    offset += limit;
                    page_count += 1;

                    // Emit progress to frontend
                    let _ = app.emit("playlists-fetch-progress", page_count * limit);

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
        // Get the current time in UTC
        let now: DateTime<Utc> = Utc::now();
        // Formats like this: 2025-08-07T12:01:00Z, which is like spotify
        let iso_8601_z_suffix = format!("{}", now.format("%+"));
        let export = PlaylistsExport {
            fetched_at: iso_8601_z_suffix,
            playlist_number: (playlists.len() as u32),
            playlists: playlists.clone(),
        };

        let json_data = json!(export);

        // let mut path = PathBuf::from("../data/");
        let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/");
        fs::create_dir_all(&path).unwrap();
        path.push("userPlaylists.json");

        // Writting stuff
        let file = fs::File::create(&path).unwrap();
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &json_data).expect("Written to json file!");

        // Emit completion event
        let _ = app.emit("playlists-fetch-complete", &playlists.len());
    });
}

#[tauri::command]
pub async fn playlist_count(app: tauri::AppHandle) -> u32 {
    let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/userPlaylists.json");

    if !path.exists() {
        pull_playlists(app).await;
        path = PathBuf::from("../../Mars-Player-dl-Logic/data/userPlaylists.json");
    }

    let json_data = fs::read_to_string(path).expect("Failed to read");
    let parsed_json: Value = serde_json::from_str(&json_data).expect("Failed to parse");
    let number: u32 = match parsed_json["playlist_number"].as_u64() {
        Some(val) => val as u32,
        None => 0,
    };

    number
}

pub async fn load_playlists(app: tauri::AppHandle) -> Vec<SimplifiedPlaylist> {
    let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/userPlaylists.json");

    if !path.exists() {
        pull_playlists(app).await;
        path = PathBuf::from("../../Mars-Player-dl-Logic/data/userPlaylists.json");
    }

    let data = fs::read_to_string(path).unwrap();
    let export: PlaylistsExport = serde_json::from_str(&data).unwrap();
    export.playlists
}

#[tauri::command(rename_all = "snake_case")]
pub async fn playlist_page(
    app: tauri::AppHandle,
    offset: usize,
    limit: usize,
) -> Vec<SimplifiedPlaylist> {
    let songs = load_playlists(app).await;
    songs.into_iter().skip(offset).take(limit).collect()
}
