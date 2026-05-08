use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;
use crate::utils::package_json::add_dev_dependency;

pub fn setup_tailwind(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Adding Tailwind CSS...", style("•").blue());

    add_dev_dependency(&config.project_path, "tailwindcss", "^4.2.4")?;
    add_dev_dependency(&config.project_path, "@tailwindcss/vite", "^4.2.4")?;

    update_vite_config(config)?;
    write_tailwind_styles(config)?;

    Ok(())
}

fn update_vite_config(config: &ReactSetupConfig) -> Result<()> {
    write_file(
        config.project_path.join("vite.config.ts"),
        r#"import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
});
"#,
    )
}

fn write_tailwind_styles(config: &ReactSetupConfig) -> Result<()> {
    write_file(
        config.project_path.join("src/index.css"),
        r#"@import "tailwindcss";

@custom-variant dark (&:where(.dark, .dark *));

@theme {
  --font-sans: Inter, ui-sans-serif, system-ui, sans-serif;

  --color-background: #ffffff;
  --color-foreground: #111827;
}

:root {
  color: var(--color-foreground);
  background: var(--color-background);
  font-family: var(--font-sans);
}

.dark {
  --color-background: #0f172a;
  --color-foreground: #f8fafc;
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
    )
}

fn write_file(path: PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}