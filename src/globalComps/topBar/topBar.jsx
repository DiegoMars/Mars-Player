import { useNavigate, useLocation } from "react-router-dom";
import styles from './topBar.module.css';

function TopBar() {
  const navigate = useNavigate();
  const location = useLocation();
  const canGoBack = location.key === 'default';

  return (
    <section className={styles.topBar}>
      <div className={styles.arrows}>
        <button onClick={() => navigate(-1)} disabled={canGoBack}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M512 256a256 256 0 1 0 -512 0 256 256 0 1 0 512 0zM271 135c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-87 87 87 87c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L167 273c-9.4-9.4-9.4-24.6 0-33.9L271 135z"/>
          </svg>
        </button>
        <button onClick={() => navigate(1)}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M0 256a256 256 0 1 0 512 0 256 256 0 1 0 -512 0zM241 377c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l87-87-87-87c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0L345 239c9.4 9.4 9.4 24.6 0 33.9L241 377z"/>
          </svg>
        </button>
      </div>
    </section>
  );
}

export default TopBar;
