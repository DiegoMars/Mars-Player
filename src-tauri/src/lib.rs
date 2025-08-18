mod spotify;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            spotify::tracks::liked_songs::pull_songs,
            spotify::tracks::liked_songs::song_count,
            spotify::tracks::liked_songs::songs_page,
            spotify::tracks::user_playlists::pull_playlists,
            spotify::tracks::user_playlists::playlist_count,
            spotify::tracks::user_playlists::playlist_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
