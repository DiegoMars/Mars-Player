import { Link } from "react-router-dom";
import styles from './directory.module.css';

function Directory() {
  return (
    <nav className={styles.directory}>
      <Link to="/">
        <h4>Home</h4>
      </Link>
      <Link to="/likedSongs">
        <h5>Liked Songs</h5>
      </Link>
      <Link to="/userPlaylists">
        <h5>Your Playlists</h5>
      </Link>
    </nav>
  );
}

export default Directory;
