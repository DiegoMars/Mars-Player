import { useNavigate, useLocation } from "react-router-dom";
import styles from './topBar.module.css';

function TopBar() {
  const navigate = useNavigate();
  const location = useLocation();
  const canGoBack = location.key === 'default';
  // Need to figure out disabling the forward feature when appropriate

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
      <div className={styles.searchBar}>
        <svg className={styles.icon} aria-hidden="true" viewBox="0 0 24 24">
        <g><path d="M21.53 20.47l-3.66-3.66C19.195 15.24 20 13.214 20 11c0-4.97-4.03-9-9-9s-9 4.03-9 9 4.03 9 9 9c2.215 0 4.24-.804 5.808-2.13l3.66 3.66c.147.146.34.22.53.22s.385-.073.53-.22c.295-.293.295-.767.002-1.06zM3.5 11c0-4.135 3.365-7.5 7.5-7.5s7.5 3.365 7.5 7.5-3.365 7.5-7.5 7.5-7.5-3.365-7.5-7.5z"></path></g>
        </svg>
        <input placeholder="Search" type="search" className={styles.input}></input>
      </div>
    </section>
  );
}

export default TopBar;
