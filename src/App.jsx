import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import RootLayout from "./layout/RootLayout";
import LikedSongs from "./pages/likedSongs/likedSongs.jsx";
import "./App.css";

function App() {
  return (
    <Router>
      <Routes>
        {/* Layout Route */}
        <Route element={<RootLayout />}>
          {/* Here is for the main page <Route path="/" element={<Home />} /> */}
          <Route path="/" element={<LikedSongs />} />
          <Route path="/likedSongs" element={<LikedSongs />} />
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
