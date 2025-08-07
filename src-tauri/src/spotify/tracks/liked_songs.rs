use crate::spotify::auth::get_spotify;
use chrono::{DateTime, Utc};
use rspotify::{
    model::{Market, SavedTrack},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, io, path::PathBuf};
use tauri::Emitter;

#[derive(Serialize, Deserialize)]
struct LikedSongsExport {
    fetched_at: String,
    song_number: u32,
    songs: Vec<SavedTrack>,
}

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
        // Get the current time in UTC
        let now: DateTime<Utc> = Utc::now();
        // Formats like this: 2025-08-07T12:01:00Z, which is like spotify
        let iso_8601_z_suffix = format!("{}", now.format("%+"));
        let export = LikedSongsExport {
            fetched_at: iso_8601_z_suffix,
            song_number: (liked_songs.len() as u32),
            songs: liked_songs.clone(),
        };

        let json_data = json!(export);

        // let mut path = PathBuf::from("../data/");
        let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/");
        fs::create_dir_all(&path).unwrap();
        path.push("liked_songs.json");

        // Writting stuff
        let file = fs::File::create(&path).unwrap();
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &json_data).expect("Written to json file!");

        // Emit completion event
        let _ = app.emit("fetch-complete", &liked_songs.len());
    });
}

#[tauri::command]
pub async fn song_count(app: tauri::AppHandle) -> u32 {
    let mut path = PathBuf::from("../../Mars-Player-dl-Logic/data/liked_songs.json");

    if !path.exists() {
        pull_songs(app).await;
        path = PathBuf::from("../../Mars-Player-dl-Logic/data/liked_songs.json");
    }

    let json_data = fs::read_to_string(path).expect("Failed to read"); // Replace with your file path
    let parsed_json: Value = serde_json::from_str(&json_data).expect("Failed to parse");
    let number: u32 = match parsed_json["song_number"].as_u64() {
        Some(val) => val as u32,
        None => 0,
    };

    number
}
