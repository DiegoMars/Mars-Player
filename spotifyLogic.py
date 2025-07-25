import os
import spotipy
import json
from spotipy.oauth2 import SpotifyOAuth
from dotenv import load_dotenv

# Loads secrets
load_dotenv(dotenv_path=os.path.expanduser("../Mars-Player-dl-Logic/.env"))

# Sets the scope of what to retrieve
scope = "user-library-read playlist-read-private"

# Creates spotipy object
sp = spotipy.Spotify(auth_manager=SpotifyOAuth(client_id=os.getenv("SPOTIPY_CLIENT_ID"),
                                               client_secret=os.getenv(
                                                   "SPOTIPY_CLIENT_SECRET"),
                                               redirect_uri=os.getenv(
                                                   "SPOTIPY_REDIRECT_URI"),
                                               scope=scope
                                               ))


def retrieveLikedSongs():
    all_songs = []
    offset = 0
    limit = 50

    while True:
        print(f"Grabbing song from song #{offset}")
        results = sp.current_user_saved_tracks(limit=limit, offset=offset)
        items = results.get("items", [])
        if not items:
            break

        for item in items:
            track = item["track"]
            cleaned = {
                "album": {
                    "album_type": track["album"]["album_type"],
                    "total_tracks": track["album"]["total_tracks"],
                    "images": track["album"]["images"],
                    "name": track["album"]["name"],
                    "release_date": track["album"]["release_date"],
                    "artists": track["album"]["artists"],
                },
                "artists": [artist["name"] for artist in track["artists"]],
                "name": track["name"],
                "duration": track["duration_ms"],
                "explicit": track["explicit"],
                "id": track["id"],
                "spotify_url": track["external_urls"]["spotify"]
            }
            all_songs.append(cleaned)
        offset += limit
    return all_songs


data = retrieveLikedSongs()
with open("userData/user_data.json", "w") as f:
    json.dump(data, f, indent=2)
