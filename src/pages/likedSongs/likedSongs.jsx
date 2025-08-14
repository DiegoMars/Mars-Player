import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import styles from './likedSongs.module.css';

function LikedSongs() {
  const [progress, setProgress] = useState(null);
  const [doneCount, setDoneCount] = useState(null);
  const [songCount, setSongCount] = useState(0);
  const [offset, setOffset] = useState(0);
  const [pages, setPages] = useState([]);

  useEffect(() => { // Json refresh
    const unlistenProgress = listen("fetch-progress", (event) => {
      setProgress(event.payload);
      setDoneCount(null);
    });

    const unlistenComplete = listen("fetch-complete", (event) => {
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
    const count = await invoke("song_count");
    setSongCount(count);

    const data = await invoke("songs_page", { offset: offset, limit: 50 });
    setPages(data);
    console.log(pages);
  }

  async function pull_songs() { // Grabs all songs into a json file
    invoke("pull_songs");
  }

  function convertMS(ms) {
    const result = `${Math.floor(ms / 60000)}:${String(Math.floor((ms % 60000) / 1000)).padStart(2, '0')}`;
    return result;
  }

  return (
    <section className={styles.likedSongs}>
      <div className={styles.header}>
        <h3>Liked Songs</h3>
        <button type="button" className={styles.button} onClick={pull_songs} disabled={progress !== null && doneCount === null}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" className={progress !== null && doneCount === null ? styles.spin : ""}>
            <path d="M436.7 74.7L448 85.4 448 32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 128c0 17.7-14.3 32-32 32l-128 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l47.9 0-7.6-7.2c-.2-.2-.4-.4-.6-.6-75-75-196.5-75-271.5 0s-75 196.5 0 271.5 196.5 75 271.5 0c8.2-8.2 15.5-16.9 21.9-26.1 10.1-14.5 30.1-18 44.6-7.9s18 30.1 7.9 44.6c-8.5 12.2-18.2 23.8-29.1 34.7-100 100-262.1 100-362 0S-25 175 75 75c99.9-99.9 261.7-100 361.7-.3z"/>
          </svg>
        </button>
        {progress !== null && doneCount === null && <p className={styles.progress}>Progress: {progress} songs</p>}
        {doneCount !== null && <p className={styles.done}>Finished! Total: {doneCount}</p>}
      </div>
      <h5>{songCount} liked songs</h5>
      <table className={styles.listOfSongs}>
        <tbody>
          <tr>
            <th>#</th>
            <th className={styles.titleH}>Title</th>
            <th className={styles.albumH}>Album</th>
            <th className={styles.dateH}>Date Added</th>
            <th className={styles.lengthH}>Length</th>
          </tr>
          {pages.map((Song, idx) => (
            <tr className={styles.song} key={idx}>
              <td className={styles.num}>{idx + 1}</td>
              <td>
                <div className={styles.title}>
                  <img src={Song.track.album.images[2].url} height={Song.track.album.images[2].height} width={Song.track.album.images[2].width} />
                  <div className={styles.names}>
                    <b>{Song.track.name}</b>
                    <p>{Song.track.artists.map(artist => artist.name).join(", ")}</p>
                  </div>
                </div>
              </td>
              <td>
                <p>{Song.track.album.name}</p>
              </td>
              <td>
                <p>{new Date(Song.added_at).toLocaleString("en-US", { month: "short", day: "2-digit", year: "numeric"} )}</p>
              </td>
              <td>
                <p>{convertMS(Song.track.duration_ms)}</p>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

export default LikedSongs;
