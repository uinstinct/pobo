import React from "react";
import ReactDOM from "react-dom/client";
import "./global.css";
import App from "./screens/main";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
