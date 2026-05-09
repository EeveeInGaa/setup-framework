use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::{Map, Value};

pub fn add_dependency(project_path: &Path, name: &str, version: &str) -> Result<()> {
    add_package(project_path, "dependencies", name, version)
}

pub fn add_dev_dependency(project_path: &Path, name: &str, version: &str) -> Result<()> {
    add_package(project_path, "devDependencies", name, version)
}

pub fn add_script(project_path: &Path, name: &str, command: &str) -> Result<()> {
    let package_json_path = project_path.join("package.json");

    let content = fs::read_to_string(&package_json_path)
        .with_context(|| format!("Failed to read '{}'", package_json_path.display()))?;

    let mut package_json: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse '{}'", package_json_path.display()))?;

    let root = package_json
        .as_object_mut()
        .context("package.json root must be an object")?;

    let scripts_value = root
        .entry("scripts".to_string())
        .or_insert_with(|| Value::Object(Map::new()));

    let scripts = scripts_value
        .as_object_mut()
        .context("'scripts' in package.json must be an object")?;

    scripts.insert(
        name.to_string(),
        Value::String(command.to_string()),
    );

    let formatted = serde_json::to_string_pretty(&package_json)
        .context("Failed to format package.json")?;

    fs::write(&package_json_path, format!("{formatted}\n"))
        .with_context(|| format!("Failed to write '{}'", package_json_path.display()))?;

    Ok(())
}

fn add_package(project_path: &Path, section: &str, name: &str, version: &str) -> Result<()> {
    let package_json_path = project_path.join("package.json");

    let content = fs::read_to_string(&package_json_path)
        .with_context(|| format!("Failed to read '{}'", package_json_path.display()))?;

    let mut package_json: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse '{}'", package_json_path.display()))?;

    let root = package_json
        .as_object_mut()
        .context("package.json root must be an object")?;

    let section_value = root
        .entry(section.to_string())
        .or_insert_with(|| Value::Object(Map::new()));

    let dependencies = section_value
        .as_object_mut()
        .with_context(|| format!("'{section}' in package.json must be an object"))?;

    dependencies.insert(name.to_string(), Value::String(version.to_string()));

    let formatted = serde_json::to_string_pretty(&package_json)
        .context("Failed to format package.json")?;

    fs::write(&package_json_path, format!("{formatted}\n"))
        .with_context(|| format!("Failed to write '{}'", package_json_path.display()))?;

    Ok(())
}