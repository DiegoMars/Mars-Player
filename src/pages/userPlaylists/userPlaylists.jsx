import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import styles from './userPlaylists.module.css';

function UserPlaylists() {
  const [progress, setProgress] = useState(null);
  const [doneCount, setDoneCount] = useState(null);
  const [playlistCount, setPlaylistCount] = useState(0);
  const [offset, setOffset] = useState(0);
  const [pages, setPages] = useState([]);

  useEffect(() => { // Json refresh
    const unlistenProgress = listen("playlists-fetch-progress", (event) => {
      setProgress(event.payload);
      setDoneCount(null);
    });

    const unlistenComplete = listen("playlists-fetch-complete", (event) => {
      setDoneCount(event.payload);
      grabPage()
    });

    // Cleanup function
    return () => {
      unlistenProgress.then((f) => f());
      unlistenComplete.then((f) => f());
    };
  }, []);

  useEffect(() => { // Loads when the page loads, and when offset changes
    grabPage();
  }, [offset]);

  async function grabPage() { // grabs a limit set of songs from json file
    const count = await invoke("playlist_count");
    setPlaylistCount(count);

    const data = await invoke("playlist_page", { offset: offset, limit: 50 });
    setPages(data);
  }

  async function pull_playlists() { // Grabs all songs into a json file
    invoke("pull_playlists");
  }

  console.log(pages);
  return (
    <section className={styles.playLists}>
      <div className={styles.header}>
        <h3>Your Playlists</h3>
        <button type="button" className={styles.button} onClick={pull_playlists} disabled={progress !== null && doneCount === null}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" className={progress !== null && doneCount === null ? styles.spin : ""}>
            <path d="M436.7 74.7L448 85.4 448 32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 128c0 17.7-14.3 32-32 32l-128 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l47.9 0-7.6-7.2c-.2-.2-.4-.4-.6-.6-75-75-196.5-75-271.5 0s-75 196.5 0 271.5 196.5 75 271.5 0c8.2-8.2 15.5-16.9 21.9-26.1 10.1-14.5 30.1-18 44.6-7.9s18 30.1 7.9 44.6c-8.5 12.2-18.2 23.8-29.1 34.7-100 100-262.1 100-362 0S-25 175 75 75c99.9-99.9 261.7-100 361.7-.3z"/>
          </svg>
        </button>
        {progress !== null && doneCount === null && <p className={styles.progress}>Progress: {progress} songs</p>}
        {doneCount !== null && <p className={styles.done}>Finished! Total: {doneCount}</p>}
      </div>
      <h5>{playlistCount.toLocaleString('en-US')} playlists songs</h5>
      <div className={styles.list}>
        {pages.map((Playlist, idx) => (
          <a className={styles.playlist} id={idx} href={Playlist.external_urls.spotify}>
            <img src={Playlist.images?.[1]?.url || Playlist.images?.[0]?.url || "https://ionicframework.com/docs/img/demos/thumbnail.svg"} height={225} width={225} alt={Playlist.name}/>
            <div className={styles.description}>
              <b>{Playlist.name}</b>
              <p>By {Playlist.owner.display_name}</p>
            </div>
          </a>
        ))}
      </div>
    </section>
  );
}

export default UserPlaylists;
