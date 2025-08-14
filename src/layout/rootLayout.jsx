import { Outlet } from "react-router-dom";
import Directory from "../globalComps/directory/directory.jsx";
import TopBar from "../globalComps/topBar/topBar.jsx"
import styles from "./rootLayout.module.css"

function RootLayout() {
  return (
    <div className={styles.appLayout}>
      <div className={styles.topBar}>
        <TopBar />
      </div>
      <div className={styles.stack}>
        <Directory className={styles.directory} />
        <div className={styles.content}>
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export default RootLayout;
