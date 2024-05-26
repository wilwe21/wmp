import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import "./scss/main.scss";
import App from "./modules/App.js";
import Video from "./modules/video.js";
import Fi from "./modules/fi.js";

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
	<Fi />
	<Video />
  </React.StrictMode>
);
