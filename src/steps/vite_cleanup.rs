use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;

pub fn cleanup_vite_template(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Cleaning up Vite template...", style("•").blue());

    write_file(
        config.project_path.join("src/router.tsx"),
        r#"import { createBrowserRouter } from "react-router";
import { RootLayout } from "./core/RootLayout";
import { Home } from "./pages/general/Home";
import { NotFound } from "./pages/general/NotFound";

export const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    errorElement: <NotFound />,
    children: [
      { index: true, element: <Home />, handle: { title: 'Home' }},
      {
        path: 'terms',
        lazy: async () => {
           const m = await import('./pages/general/Terms');
           return { Component: m.Terms };
        },
        handle: { title: 'Terms & Conditions' }
      },
      {
        path: 'imprint',
        lazy: async () => {
           const m = await import('./pages/general/Imprint');
           return { Component: m.Imprint };
        },
        handle: { title: 'Imprint' }
      },
      {
        path: 'privacy',
        lazy: async () => {
           const m = await import('./pages/general/Privacy');
           return { Component: m.Privacy };
        },
        handle: { title: 'Privacy Policy' }
      },
    ],
  },
]);
"#,
    )?;

    write_file(
        config.project_path.join("src/main.tsx"),
        r#"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from "react-router";
import { router } from "./router";
import './index.css'

const rootElement = document.getElementById("root");

if (!rootElement) {
  throw new Error("Root element not found");
}

createRoot(rootElement).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
);
"#,
    )?;

    remove_file_if_exists(config.project_path.join("src/App.css"))?;
    remove_file_if_exists(config.project_path.join("src/App.tsx"))?;

    Ok(())
}

fn write_file(path: PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}

fn remove_file_if_exists(path: PathBuf) -> Result<()> {
    if Path::new(&path).exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to remove file '{}'", path.display()))?;
    }

    Ok(())
}