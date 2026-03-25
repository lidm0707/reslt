use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Component metadata from the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub variant: String,
    pub style: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub files: Vec<String>,
    pub dependencies: HashMap<String, String>,
    pub requires_tailwind: bool,
    #[serde(skip)]
    pub full_name: String,
}

/// Registry containing available components
#[derive(Debug)]
pub struct Registry {
    pub components: Vec<Component>,
    pub registry_path: String,
}

/// Style choice for component installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleChoice {
    RawCss,
    Tailwind,
    None,
}

/// Options for installing a component
#[derive(Clone)]
pub struct InstallOptions {
    pub component_name: String,
    pub style: StyleChoice,
    pub output_dir: String,
    pub add_to_mod: bool,
}

/// Fetches the component registry from a local directory or URL
pub async fn fetch_registry(registry_path: &str) -> Result<Registry> {
    let path = Path::new(registry_path);

    if path.exists() && path.is_dir() {
        fetch_local_registry(registry_path)
    } else if registry_path.starts_with("http://") || registry_path.starts_with("https://") {
        fetch_remote_registry(registry_path).await
    } else {
        anyhow::bail!(
            "Registry path '{}' does not exist and is not a valid URL",
            registry_path
        )
    }
}

/// Fetches the component registry from a local directory
fn fetch_local_registry(registry_path: &str) -> Result<Registry> {
    let mut components = Vec::new();

    let entries = fs::read_dir(registry_path).context("Failed to read registry directory")?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let metadata_path = path.join("metadata.json");
        if !metadata_path.exists() {
            continue;
        }

        let metadata_content =
            fs::read_to_string(&metadata_path).context("Failed to read metadata.json")?;

        let mut component: Component =
            serde_json::from_str(&metadata_content).context("Failed to parse metadata.json")?;

        // Generate full_name from name, variant, and style
        component.full_name = format!(
            "{}_{}_{}",
            component.name, component.variant, component.style
        );

        components.push(component);
    }

    components.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Registry {
        components,
        registry_path: registry_path.to_string(),
    })
}

/// Fetches the component registry from a URL
async fn fetch_remote_registry(_registry_url: &str) -> Result<Registry> {
    anyhow::bail!(
        "Remote registry fetching is not yet implemented. Please use a local registry path."
    );
}

/// Finds a component by name in the registry
pub fn find_component(registry: &Registry, name: &str) -> Option<Component> {
    // First try to match by full directory name (name-variant-style)
    if let Some(component) = registry.components.iter().find(|c| {
        let full_name = format!("{}_{}_{}", c.name, c.variant, c.style);
        full_name.eq_ignore_ascii_case(name)
    }) {
        return Some(component.clone());
    }

    // Fall back to matching by just the name field
    registry
        .components
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case(name))
        .cloned()
}

/// Lists components in the registry
pub fn list_components(registry: &Registry) -> Vec<&Component> {
    registry.components.iter().collect()
}

/// Gets the CSS file path based on style choice
#[allow(dead_code)]
pub fn get_css_file(component: &Component, style: StyleChoice) -> Option<String> {
    match style {
        StyleChoice::RawCss if component.style == "rawcss" => {
            Some(format!("{}.css", component.name))
        }
        StyleChoice::Tailwind if component.style == "tailwind" => None, // Tailwind uses classes, no CSS file
        _ => None,
    }
}

/// Installs a component from the registry to the specified directory
pub fn install_component(registry: &Registry, options: InstallOptions) -> Result<InstallResult> {
    let output_path = Path::new(&options.output_dir);

    fs::create_dir_all(output_path).context("Failed to create output directory")?;

    let component = find_component(registry, &options.component_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Component '{}' not found in registry",
            options.component_name
        )
    })?;

    let component_dir_name = format!(
        "{}_{}_{}",
        component.name, component.variant, component.style
    );
    let component_registry_path = Path::new(&registry.registry_path).join(&component_dir_name);

    if !component_registry_path.exists() {
        anyhow::bail!(
            "Component directory '{}' not found in registry",
            component_dir_name
        );
    }

    let mut component_files = Vec::new();
    let mut css_files = Vec::new();

    for file in &component.files {
        let src_path = component_registry_path.join(file);
        let dest_path = output_path.join(file);

        if dest_path.exists() {
            anyhow::bail!("File '{}' already exists in output directory", file);
        }

        fs::copy(&src_path, &dest_path)
            .context(format!("Failed to copy file '{}' from registry", file))?;

        if file.ends_with(".css") {
            css_files.push(file.clone());
        } else {
            component_files.push(file.clone());
        }
    }

    if let StyleChoice::RawCss = options.style {
        let css_file = format!("{}.css", component.name);
        let css_src_path = component_registry_path.join(&css_file);
        if css_src_path.exists() {
            let css_dest_path = output_path.join(&css_file);
            if !css_dest_path.exists() {
                fs::copy(&css_src_path, &css_dest_path)?;
                css_files.push(css_file);
            }
        }
    }

    let mod_updated = if options.add_to_mod {
        update_mod_file(output_path, &component.name)?
    } else {
        false
    };

    Ok(InstallResult {
        component_files,
        css_files,
        mod_updated,
    })
}

/// Result of installing a component
#[derive(Debug)]
pub struct InstallResult {
    pub component_files: Vec<String>,
    pub css_files: Vec<String>,
    pub mod_updated: bool,
}

/// Updates the mod.rs file to include the new component
fn update_mod_file(dir: &Path, component_name: &str) -> Result<bool> {
    let mod_path = dir.join("mod.rs");

    let content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        String::new()
    };

    let module_name = to_snake_case(component_name);

    if content.contains(&format!("mod {};", module_name)) {
        return Ok(false);
    }

    let new_content = if content.is_empty() {
        format!("pub mod {};\n", module_name)
    } else {
        format!("{}pub mod {};\n", content, module_name)
    };

    fs::write(&mod_path, new_content)?;
    Ok(true)
}

/// Converts a string to snake_case
pub fn to_snake_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("_{}", c.to_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .trim_start_matches('_')
        .to_string()
        .replace('-', "_")
}

/// Converts a string to PascalCase
#[allow(dead_code)]
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-' || c == ' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Converts a string to kebab-case
#[allow(dead_code)]
pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}
