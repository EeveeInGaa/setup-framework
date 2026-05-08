use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;

pub fn cleanup_vite_template(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Cleaning up Vite template...", style("•").blue());

    write_file(
        config.project_path.join("src/App.tsx"),
        r#"import { Header } from "./core/Header";
import { Footer } from "./core/Footer";
import { Home } from "./pages/general/Home";

export function App() {
  return (
    <>
      <Header />
      <Home />
      <Footer />
    </>
  );
}
"#,
    )?;

    write_file(
        config.project_path.join("src/index.css"),
        r#":root {
  font-family: system-ui, sans-serif;
  color: #111827;
  background: #ffffff;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}
"#,
    )?;

    remove_file_if_exists(config.project_path.join("src/App.css"))?;

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