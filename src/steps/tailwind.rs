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

:root,
.light {
  --color-background: #f8f8f7;
  --color-primary: #a736c6;
  --color-secondary: #6f44c1;
  --color-text: #111827;
  --color-text-invert: #f8f8f7;
  --color-error: #b94b4a;
  --color-warning: #bfb133;
  --color-success: #67a448;
  --color-info: #3270c2;
}

@media (prefers-color-scheme: dark) {
  :root:not(.light) {
    --color-background: #111827;
    --color-primary: #b266c4;
    --color-secondary: #9078bd;
    --color-text: #f8f8f7;
    --color-text-invert: #111827;
    --color-error: #df5d5c;
    --color-warning: #d1c147;
    --color-success: #7dc957;
    --color-info: #3d8aef;
  }
}

.dark {
  --color-background: #111827;
  --color-primary: #b266c4;
  --color-secondary: #9078bd;
  --color-text: #f8f8f7;
  --color-text-invert: #111827;
  --color-error: #df5d5c;
  --color-warning: #d1c147;
  --color-success: #7dc957;
  --color-info: #3d8aef;
}

@theme {
  --font-sans: Inter, ui-sans-serif, system-ui, sans-serif;

  --breakpoint-*: initial;
  --breakpoint-xs: 30rem; /* 480px */
  --breakpoint-sm: 40rem; /* 640px */
  --breakpoint-md: 38rem; /* 768px */
  --breakpoint-lg: 64rem; /* 1024px */
  --breakpoint-xl: 80rem; /* 1280px */
  --breakpoint-xxl: 96rem; /* 1536px */

  --font-display: Inter, ui-sans-serif, system-ui, sans-serif;

  --text-*: initial;
  --text-xs: 0.75rem;
  --text-sm: 0.875rem;
  --text-base: 1rem;
  --text-lg: 1.125rem;
  --text-xl: 1.25rem;

  --font-weight-thin: 200;
  --font-weight-normal: 400;
  --font-weight-bold: 600;

  --shadow-small: 0 3px 6px rgba(0, 0, 0, 0.2);
  --shadow-normal: 0 4px 8px rgba(0, 0, 0, 0.2);

  --radius-*: initial;
  --radius-xs: 0.25rem;
  --radius-sm: 0.5rem;
  --radius-md: 0.75rem;
  --radius-lg: 1rem;
  --radius-xl: 1.25rem;
  --radius-xxl: 1.5rem;

  --color-*: initial;
  --color-background: var(--color-background);
  --color-primary: var(--color-primary);
  --color-secondary: var(--color-secondary);
  --color-text: var(--color-text);
  --color-text-invert: var(--color-text-invert);
  --color-error: var(--color-error);
  --color-warning: var(--color-warning);
  --color-success: var(--color-success);
  --color-info: var(--color-info);
}

@layer base {
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
}

html,
body {
  font-size: 16px;

  @apply font-sans text-text bg-background h-svh min-h-dvh w-full;
}

a {
  @apply cursor-pointer hover:underline focus:underline;
}

.custom-container {
  @apply mx-auto px-4 w-full;
}

@variant lg {
  .custom-container {
    @apply max-w-5xl;
  }
}

@variant xl {
  .custom-container {
    @apply max-w-7xl;
  }
}
"#,
    )
}

fn write_file(path: PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}