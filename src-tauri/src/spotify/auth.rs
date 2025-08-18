use dotenvy::from_path;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use std::{fs, path::PathBuf};

pub async fn get_spotify() -> AuthCodeSpotify {
    let env_path = PathBuf::from("../../Mars-Player-dl-Logic/.env");
    from_path(env_path).ok();

    let creds = Credentials::from_env().unwrap(); // Grabs ID and secret

    let scopes = scopes!("user-library-read", "playlist-read-private");
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
