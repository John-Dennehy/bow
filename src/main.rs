mod arch;
mod config;
mod provider;

use crate::provider::PackageProvider;
use clap::{Parser, Subcommand};
use miette::Result;
use owo_colors::OwoColorize;
use comfy_table::Table;

#[derive(Parser)]
#[command(name = "bow")]
#[command(about = "Put a bow on your package management.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for a package (alias: s)
    #[command(alias = "s")]
    Search {
        query: String,
    },
    /// Install packages (alias: i)
    #[command(alias = "i")]
    Install {
        packages: Vec<String>,
    },
    /// Remove packages (alias: r)
    #[command(alias = "r")]
    Uninstall {
        packages: Vec<String>,
    },
    /// Update the system (alias: u)
    #[command(alias = "u")]
    Update,
    /// Clean the system of orphans (alias: c)
    #[command(alias = "c")]
    Clean,
}

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new()
            .terminal_links(true)
            .context_lines(3)
            .tab_width(4)
            .build())
    })).unwrap();

    let cli = Cli::parse();
    let _config = config::Config::load()?;

    // Initialize provider (currently only Arch is supported)
    let provider = arch::ArchProvider::new();
    println!("{} Using provider: {}", "=>".green().bold(), provider.name().blue());

    match cli.command {
        Commands::Search { query } => {
            let results = provider.search(&query)?;
            
            let mut table = Table::new();
            table.set_header(vec!["Repository", "Name", "Version", "Description", "Status"]);
            
            for pkg in results {
                let status = if pkg.is_installed { "Installed".green().to_string() } else { "Not Installed".red().to_string() };
                table.add_row(vec![
                    pkg.repository.blue().to_string(),
                    pkg.name.bold().to_string(),
                    pkg.version.yellow().to_string(),
                    pkg.description,
                    status,
                ]);
            }
            
            println!("{table}");
        }
        Commands::Install { packages } => {
            println!("{} Installing packages: {:?}", "=>".green().bold(), packages);
            provider.install(&packages)?;
        }
        Commands::Uninstall { packages } => {
            println!("{} Removing packages: {:?}", "=>".red().bold(), packages);
            provider.remove(&packages)?;
        }
        Commands::Update => {
            println!("{} Updating system", "=>".blue().bold());
            provider.update()?;
        }
        Commands::Clean => {
            println!("{} Cleaning system of orphans and cache", "=>".yellow().bold());
            provider.clean()?;
        }
    }

    Ok(())
}
