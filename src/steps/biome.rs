use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;
use crate::utils::package_json::{add_dev_dependency, add_script};

pub fn setup_biome(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Adding Biome...", style("•").blue());

    add_dev_dependency(
        &config.project_path,
        "@biomejs/biome",
        "^2.4.14",
    )?;

    add_script(
        &config.project_path,
        "format",
        "biome format --write .",
    )?;

    add_script(
        &config.project_path,
        "lint",
        "biome lint --write .",
    )?;

    add_script(
        &config.project_path,
        "lint-unsafe",
        "biome lint --write --unsafe .",
    )?;

    add_script(
        &config.project_path,
        "check",
        "biome check --write .",
    )?;

    write_biome_config(config)?;

    Ok(())
}

fn write_biome_config(config: &ReactSetupConfig) -> Result<()> {
    write_file(
        config.project_path.join("biome.json"),
        r#"{
	"$schema": "https://biomejs.dev/schemas/2.4.14/schema.json",
	"vcs": {
		"enabled": true,
		"clientKind": "git",
		"useIgnoreFile": true
	},
	"formatter": {
		"enabled": true,
		"indentStyle": "tab"
	},
	"linter": {
		"enabled": true,
		"rules": {
			"recommended": true,
			"correctness": {
                "noUnusedImports": "error",
                "noUnusedVariables": "warn"
            }
		},
		"domains": {
			"react": "recommended"
		}
	},
	"javascript": {
		"formatter": {
			"quoteStyle": "single"
		}
	},
	"json": {
		"parser": {
			"allowComments": true
		}
	},
	"css": {
		"parser": {
			"cssModules": true,
			"tailwindDirectives": true
		}
	},
	"assist": {
		"actions": {
			"source": {
				"organizeImports": {
					"options": {
						"groups": [
							[
								":BUN:",
								":NODE:"
							],
							":BLANK_LINE:",
							[
								":PACKAGE:",
								"!@myown/**"
							],
							":BLANK_LINE:",
							"@myown/**",
							":BLANK_LINE:",
							[
								":ALIAS:",
								":PATH:"
							],
							":BLANK_LINE:",
							[
								"**",
								"!**/*.css",
								"!**/*.scss"
							],
							":BLANK_LINE:",
							[
								"**/*.css",
								"**/*.scss"
							]
						]
					}
				}
			}
		}
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