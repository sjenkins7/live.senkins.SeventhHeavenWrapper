import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./routes/root";
import ErrorPage from "./error-page";
import "./index.css";

import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import Config from "./routes/config";
import Run from "./routes/run";
import Debug from "./routes/debug";


const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/config",
    element: <Config />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/debug",
    element: <Debug />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/run",
    element: <Run />,
    errorElement: <ErrorPage />,
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <div>
      <RouterProvider router={router} />
    </div>
  </React.StrictMode>,
);
