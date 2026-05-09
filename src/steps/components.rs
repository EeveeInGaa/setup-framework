use std::fs;

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;

pub fn create_basic_components(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Creating basic components...", style("•").blue());

    let components_path = config.project_path.join("src/components");
    let core_path = config.project_path.join("src/core");
    let pages_path = config.project_path.join("src/pages/general");

    fs::create_dir_all(&components_path)
        .with_context(|| format!("Failed to create '{}'", components_path.display()))?;

    fs::create_dir_all(&core_path)
        .with_context(|| format!("Failed to create '{}'", core_path.display()))?;

    fs::create_dir_all(&pages_path)
        .with_context(|| format!("Failed to create '{}'", pages_path.display()))?;

    write_file(
        components_path.join("Button.tsx"),
        r#"
  export function Button({ children, onClick, ...props }) {
    return (
      <button type="button" onClick={onClick} {...props}>
        {children}
      </button>
    );
  }
  "#,
    )?;

    write_file(
        core_path.join("Header.tsx"),
        r#"export function Header() {
    return (
      <header>
        <p>Header</p>
      </header>
    );
  }
  "#,
    )?;

    write_file(
        core_path.join("Footer.tsx"),
        r#"export function Footer() {
    return (
      <footer>
        <p>Footer</p>
      </footer>
    );
  }
  "#,
    )?;

    write_file(
        core_path.join("RootLayout.tsx"),
        r#"import { Outlet, useMatches, type UIMatch } from "react-router-dom";
import { Header } from "./Header";
import { Footer } from "./Footer";
import { useEffect } from "react";

type RouteHandle = {
    title?: string;
};

export function RootLayout() {
    const matches = useMatches() as UIMatch<unknown, RouteHandle>[];

    useEffect(() => {
        const match = [...matches].reverse().find((match) => match.handle?.title);
        const title = match?.handle?.title;

        document.title = title ?? "My App";
    }, [matches]);

    return (
        <>
            <Header />
            <main><Outlet /></main>
            <Footer />
        </>
    );
}
"#,
    )?;

    write_file(
        pages_path.join("Home.tsx"),
        r#"export function Home() {
    return <h1>Home</h1>;
  }
  "#,
    )?;

    write_file(
        pages_path.join("Terms.tsx"),
        r#"export function Terms() {
    return <h1>Terms</h1>;
  }
  "#,
    )?;

    write_file(
        pages_path.join("Imprint.tsx"),
        r#"export function Imprint() {
    return <h1>Imprint</h1>;
  }
  "#,
    )?;

    write_file(
        pages_path.join("Privacy.tsx"),
        r#"export function Privacy() {
    return <h1>Privacy</h1>;
  }
  "#,
    )?;

    write_file(
        pages_path.join("NotFound.tsx"),
        r#"export function NotFound() {
    return <h1>Not Found</h1>;
  }
  "#,
    )?;

    Ok(())
}

fn write_file(path: std::path::PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}