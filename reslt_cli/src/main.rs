use anyhow::Result;
use clap::Parser;

mod command;
mod core;

use command::{Commands, handle_add, handle_list, handle_search};

const DEFAULT_REGISTRY_URL: &str = "../reslt-component/registry";

#[derive(Parser, Debug)]
#[command(name = "reslt")]
#[command(about = "CLI tool for adding Reslt components from a registry", long_about = None)]
struct Cli {
    /// Registry URL to fetch components from
    #[arg(long, default_value = DEFAULT_REGISTRY_URL)]
    registry_url: String,

    #[command(subcommand)]
    commands: Commands,
}

/// Resolves and validates the registry path
/// If it's a relative path, tries to resolve it relative to the CLI binary location
/// Falls back to the current working directory if not found
fn resolve_registry_path(path: &str) -> String {
    // If it's a URL, return as-is
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }

    // Try the path as-is (could be absolute or relative to current directory)
    let path_obj = std::path::Path::new(path);
    if path_obj.exists() && path_obj.is_dir() {
        return path.to_string();
    }

    // If not found, try to resolve relative to the binary location
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let resolved = exe_dir.join(path);
            if resolved.exists() && resolved.is_dir() {
                return resolved.to_string_lossy().to_string();
            }
        }
    }

    // Return as-is and let the fetch_registry function handle errors
    path.to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Resolve and validate the registry path
    let registry_path = resolve_registry_path(&cli.registry_url);

    match cli.commands {
        Commands::List => {
            handle_list(&registry_path).await?;
        }
        Commands::Add {
            name,
            output,
            style,
            no_mod,
        } => {
            handle_add(&registry_path, name, output, style, no_mod).await?;
        }
        Commands::Search { query } => {
            handle_search(&registry_path, query).await?;
        }
    }

    Ok(())
}
