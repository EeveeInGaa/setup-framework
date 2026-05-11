use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;
use crate::utils::file::remove_file_if_exists;

pub fn setup_readme(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Creating README...", style("•").blue());
    remove_file_if_exists(config.project_path.join("README.md"))?;

    write_file(
        config.project_path.join("README.mdx"),
        &format!(
            r#"# README {}

Minimal, modern React 19 starter built with TypeScript, Vite, Tailwind CSS v4 and Biome.

Designed for fast development, clean architecture and long-term maintainability.

---

## Features

- React 19
- TypeScript
- Vite
- Tailwind CSS v4 (including design system)
- Biome (formatting + linting)
- React Router
- Modern folder structure
- Path aliases
- CSS variable based theming
- Darkmode ready
- Responsive layout foundation
- Clean and minimal setup

---

## Requirements

- Node.js 22+
- npm or pnpm

---

## Getting Started

Install dependencies (initially already done):

```bash
npm install
```

Start the development server:

```bash
npm run dev
```

Open:

```txt
http://localhost:5173
```

---

## Project Structure Overview

| Folder          | Purpose                 |
|-----------------|-------------------------|
| `components`    | Reusable UI components  |
| `core`          | Global Smart Components |
| `pages`         | Route pages             |
| `pages/feature` | Feature specific pages  |
| `pages/general` | General pages           |
| `utils`         | Utility functions       |

---

## Styling

This project uses:

- Tailwind CSS v4
- CSS variables for theming
- Token-based design system

Global theme configuration and styles are located in:

```txt
src/styles/index.css
```

---

## Routing

Routing is handled with React Router using `createBrowserRouter`.

Router configuration can be found in:
```txt
src/router.tsx
```

---

## Code Quality

This project uses **_Biome_** for:

- formatting
- linting
- organize imports

> activate **_Actions on Save_** (`Run Biome ...`) in Webstorm settigns

Biome configuration:
```txt
biome.json
```

---

## Included

- Basic **layout and styles** (removing default content)
- Common **Pages and Components** in folder structure
- **Tailwind** setup (including designsystem and darkmode)
- **Biome** setup (common linting and formatting rules)
- **React** Router setup (with titles, lazy loading and not found page)
- **git init** with main branch and **initial commit**

---

## Recommended Webstrom Extensions

- [Tailwind CSS plugin](https://plugins.jetbrains.com/plugin/15321-tailwind-css)
- [Biome Plugin](https://plugins.jetbrains.com/plugin/22761-biome)


"#,
            config.app_name
        ),
    )?;

    Ok(())
}

fn write_file(path: PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}