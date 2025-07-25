import { createRoot } from "react-dom/client";
import App from "./App";
import { StrictMode } from "react";

const root = document.getElementById("root");
if (root) {
  const rootElement = createRoot(root);
  rootElement.render(
    <StrictMode>
      <App />
    </StrictMode>
  );
}