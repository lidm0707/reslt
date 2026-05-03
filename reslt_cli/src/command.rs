use anyhow::Result;
use clap::Subcommand;
use dialoguer::{Confirm, FuzzySelect, Select, theme::ColorfulTheme};

use crate::core::{
    Component, InstallOptions, InstallResult, Registry, StyleChoice, fetch_registry,
    find_component, list_components,
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available components from the registry
    List,
    /// Add a component from the registry
    Add {
        /// Name of the component to add
        name: Option<String>,
        /// Output directory for the component (default: ./src/components)
        #[clap(short, long, default_value = "./src/components")]
        output: String,
        /// Style choice (raw-css, tailwind, none)
        #[clap(short, long)]
        style: Option<String>,
        /// Don't add to mod.rs automatically
        #[clap(long)]
        no_mod: bool,
    },
    /// Search for components by name
    Search {
        /// Search query
        query: String,
    },
}

/// Handles the List command
pub async fn handle_list(registry_url: &str) -> Result<()> {
    println!("📦 Fetching component registry...\n");

    let registry = fetch_registry(registry_url).await?;
    let components = list_components(&registry);

    if components.is_empty() {
        println!("No components found in the registry.");
        return Ok(());
    }

    println!("Available Components ({})\n", components.len());
    println!(
        "┌─{}─┬─{}─┬─{}─┬─{}─┐",
        "─".repeat(30),
        "─".repeat(10),
        "─".repeat(20),
        "─".repeat(40)
    );
    println!(
        "│ {:<30} │ {:<10} │ {:<20} │ {:<40} │",
        "Component", "Version", "Category", "Description"
    );
    println!(
        "├─{}─┼─{}─┼─{}─┼─{}─┤",
        "─".repeat(30),
        "─".repeat(10),
        "─".repeat(20),
        "─".repeat(40)
    );

    for component in components {
        println!(
            "│ {:<30} │ {:<10} │ {:<20} │ {:<40} │",
            component.full_name,
            component.version,
            component.category,
            truncate(&component.description, 40)
        );
    }

    println!(
        "└─{}─┴─{}─┴─{}─┴─{}─┘",
        "─".repeat(30),
        "─".repeat(10),
        "─".repeat(20),
        "─".repeat(40)
    );

    Ok(())
}

/// Handles the Add command
pub async fn handle_add(
    registry_url: &str,
    name: Option<String>,
    output: String,
    style: Option<String>,
    no_mod: bool,
) -> Result<()> {
    println!("📦 Fetching component registry...\n");
    let registry = fetch_registry(registry_url).await?;

    let component = get_component_from_registry(&registry, name)?;
    display_component_info(&component);

    if !confirm_installation(&component.name)? {
        println!("Installation cancelled.");
        return Ok(());
    }

    let install_context = prepare_installation(&component, output, style, !no_mod)?;
    execute_installation(&registry, &component, &install_context)?;

    Ok(())
}

/// Gets the component from the registry by name or interactive selection
fn get_component_from_registry(
    registry: &crate::core::Registry,
    name: Option<String>,
) -> Result<Component> {
    let component_name = match name {
        Some(n) => n,
        None => select_component_interactively(registry)?,
    };

    find_component(registry, &component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found in registry", component_name))
}

/// Selects a component interactively from the registry
fn select_component_interactively(registry: &crate::core::Registry) -> Result<String> {
    let components = list_components(registry);
    if components.is_empty() {
        anyhow::bail!("No components available in the registry");
    }

    let items: Vec<String> = components
        .iter()
        .map(|c| format!("{} - {}", c.full_name, c.category))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a component to add")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(components[selection].full_name.clone())
}

/// Displays component information
fn display_component_info(component: &Component) {
    println!("\n✨ Found: {}", component.full_name);
    println!("   Version: {}", component.version);
    println!("   Description: {}\n", component.description);
}

/// Confirms installation with the user
fn confirm_installation(component_name: &str) -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Install '{}'?", component_name))
        .default(true)
        .interact()?)
}

/// Gets the style choice either from arguments or interactive selection
fn get_style_choice(style: Option<String>) -> Result<StyleChoice> {
    match style {
        Some(s) => parse_style_argument(&s),
        None => select_style_interactively(),
    }
}

/// Parses the style argument string into a StyleChoice
fn parse_style_argument(s: &str) -> Result<StyleChoice> {
    match s.to_lowercase().as_str() {
        "raw-css" | "raw" | "css" => Ok(StyleChoice::RawCss),
        "tailwind" | "tw" => Ok(StyleChoice::Tailwind),
        "none" => Ok(StyleChoice::None),
        _ => anyhow::bail!(
            "Invalid style choice: {}. Use 'raw-css', 'tailwind', or 'none'",
            s
        ),
    }
}

/// Selects a style interactively
fn select_style_interactively() -> Result<StyleChoice> {
    let items = vec!["Raw CSS", "Tailwind CSS", "No styling"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select styling approach")
        .items(&items)
        .default(1)
        .interact()?;

    Ok(match selection {
        0 => StyleChoice::RawCss,
        1 => StyleChoice::Tailwind,
        _ => StyleChoice::None,
    })
}

/// Prepares the installation context with all options
fn prepare_installation(
    component: &Component,
    output: String,
    style: Option<String>,
    add_to_mod: bool,
) -> Result<InstallContext> {
    let style_choice = get_style_choice(style)?;
    println!("\n🎨 Style choice: {:?}", style_choice);

    let options = InstallOptionsBuilder::new()
        .component_name(component.name.clone())
        .style(style_choice)
        .output_dir(output)
        .add_to_mod(add_to_mod)
        .build()
        .map_err(|e| anyhow::anyhow!("Invalid install options: {}", e))?;

    Ok(InstallContext {
        options,
        component_name: component.name.clone(),
        _style_choice: style_choice,
    })
}

/// Executes the installation and displays results
fn execute_installation(
    registry: &Registry,
    component: &Component,
    ctx: &InstallContext,
) -> Result<()> {
    println!("\n🔧 Installing component...\n");
    let result = crate::core::install_component(registry, ctx.options.clone())?;
    display_installation_result(component, &result, &ctx.component_name);
    Ok(())
}

/// Installation context containing all necessary data
struct InstallContext {
    options: InstallOptions,
    component_name: String,
    _style_choice: StyleChoice,
}

/// Displays the installation result
fn display_installation_result(
    component: &Component,
    result: &InstallResult,
    component_name: &str,
) {
    println!("✅ Successfully installed '{}'", component.name);

    println!("   Component files:");
    for file in &result.component_files {
        println!("      - {}", file);
    }

    if !result.css_files.is_empty() {
        println!("   CSS files:");
        for file in &result.css_files {
            println!("      - {}", file);
        }
    }

    if result.mod_updated {
        println!("   Updated mod.rs");
    }

    println!("\n💡 Next steps:");
    println!("   1. Import and use the component in your app:");
    println!(
        "      use your_crate::components::{};",
        crate::core::to_snake_case(component_name)
    );
    println!("   2. If using Raw CSS, include the CSS file in your HTML or build process.");
}

/// Handles the Search command
pub async fn handle_search(registry_url: &str, query: String) -> Result<()> {
    println!("🔍 Searching for '{}'...\n", query);

    let registry = fetch_registry(registry_url).await?;
    let query_lower = query.to_lowercase();

    let matches: Vec<_> = registry
        .components
        .iter()
        .filter(|c| {
            c.full_name.to_lowercase().contains(&query_lower)
                || c.name.to_lowercase().contains(&query_lower)
                || c.description.to_lowercase().contains(&query_lower)
                || c.category.to_lowercase().contains(&query_lower)
        })
        .collect();

    if matches.is_empty() {
        println!("No components found matching '{}'.", query);
        println!("Use 'reslt list' to see all available components.");
        return Ok(());
    }

    println!("Found {} matching component(s):\n", matches.len());

    for component in matches {
        println!("📦 {} (v{})", component.full_name, component.version);
        println!("   Category: {}", component.category);
        println!("   Description: {}", component.description);
        println!();
    }

    Ok(())
}

/// Truncates a string to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Builder for InstallOptions (simple implementation)
pub struct InstallOptionsBuilder {
    component_name: Option<String>,
    style: Option<StyleChoice>,
    output_dir: Option<String>,
    add_to_mod: Option<bool>,
}

impl InstallOptionsBuilder {
    pub fn new() -> Self {
        Self {
            component_name: None,
            style: None,
            output_dir: None,
            add_to_mod: None,
        }
    }

    pub fn component_name(mut self, name: String) -> Self {
        self.component_name = Some(name);
        self
    }

    pub fn style(mut self, style: StyleChoice) -> Self {
        self.style = Some(style);
        self
    }

    pub fn output_dir(mut self, dir: String) -> Self {
        self.output_dir = Some(dir);
        self
    }

    pub fn add_to_mod(mut self, add: bool) -> Self {
        self.add_to_mod = Some(add);
        self
    }

    pub fn build(self) -> Result<InstallOptions> {
        Ok(InstallOptions {
            component_name: self
                .component_name
                .ok_or_else(|| anyhow::anyhow!("component_name is required"))?,
            style: self.style.unwrap_or(StyleChoice::Tailwind),
            output_dir: self
                .output_dir
                .unwrap_or_else(|| "./src/components".to_string()),
            add_to_mod: self.add_to_mod.unwrap_or(true),
        })
    }
}

impl Default for InstallOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
