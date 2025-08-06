import { Outlet } from "react-router-dom";
import Directory from "../globalComps/directory/directory.jsx";
import TopBar from "../globalComps/topBar/topBar.jsx"
import styles from "./rootLayout.module.css"

function RootLayout() {
  return (
    <div className={styles.appLayout}>
      <TopBar />
      <div className={styles.stack}>
        <Directory className={styles.directory} />
        <Outlet />
      </div>
    </div>
  );
}

export default RootLayout;
