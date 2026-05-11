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
	--background: #f8f8f7;
	--primary: #a736c6;
	--secondary: #6f44c1;
	--text: #111827;
	--text-invert: #f8f8f7;
	--error: #b94b4a;
	--warning: #bfb133;
	--success: #67a448;
	--info: #3270c2;
}

@media (prefers-color-scheme: dark) {
	:root {
		--background: #111827;
		--primary: #b266c4;
		--secondary: #9078bd;
		--text: #f8f8f7;
		--text-invert: #111827;
		--error: #df5d5c;
		--warning: #d1c147;
		--success: #7dc957;
		--info: #3d8aef;
	}
}

.dark {
	--background: #111827;
	--primary: #b266c4;
	--secondary: #9078bd;
	--text: #f8f8f7;
	--text-invert: #111827;
	--error: #df5d5c;
	--warning: #d1c147;
	--success: #7dc957;
	--info: #3d8aef;
}

@theme inline {
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
	--color-background: var(--background);
	--color-primary: var(--primary);
	--color-secondary: var(--secondary);
	--color-text: var(--text);
	--color-text-invert: var(--text-invert);
	--color-error: var(--error);
	--color-warning: var(--warning);
	--color-success: var(--success);
	--color-info: var(--info);
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