import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import styles from './likedSongs.module.css';

function LikedSongs() {
  const [progress, setProgress] = useState(null);
  const [doneCount, setDoneCount] = useState(null);

  useEffect(() => {
    const unlistenProgress = listen("fetch-progress", (event) => {
      console.log(`Fetched ${event.payload} songs so far`);
      setProgress(event.payload);
    });

    const unlistenComplete = listen("fetch-complete", (event) => {
      console.log(`Done! Total fetched: ${event.payload}`);
      setDoneCount(event.payload);
    });

    // Cleanup function
    return () => {
      unlistenProgress.then((f) => f());
      unlistenComplete.then((f) => f());
    };
  }, []);

  async function pull_songs() {
    invoke("pull_songs");
  }

  return (
    <main className={styles.container}>
      <button type="button" onClick={pull_songs}>Pull Songs</button>
      {progress !== null && <p>Progress: {progress} songs</p>}
      {doneCount !== null && <p>Finished! Total: {doneCount}</p>}
    </main>
  );
}

export default LikedSongs;
