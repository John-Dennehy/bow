use crate::provider::{Package, PackageProvider};
use miette::{miette, Result};
use std::process::Command;

pub struct ArchProvider {
    helper: String,
}

impl ArchProvider {
    pub fn new() -> Self {
        // Simple auto-detection: prefer yay, then paru, fallback to pacman
        let helper = if is_command_available("yay") {
            "yay".to_string()
        } else if is_command_available("paru") {
            "paru".to_string()
        } else {
            "pacman".to_string()
        };

        Self { helper }
    }
}

fn is_command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

impl PackageProvider for ArchProvider {
    fn name(&self) -> &'static str {
        "Arch Linux (Pacman/AUR)"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>> {
        // In a real implementation, we would parse `pacman -Ss` or `yay -Ss`
        // Here we just simulate running the command and returning a dummy package for demo
        println!("Running: {} -Ss {}", self.helper, query);
        
        let output = Command::new(&self.helper)
            .arg("-Ss")
            .arg(query)
            .output()
            .map_err(|e| miette!("Failed to execute search: {}", e))?;

        if !output.status.success() {
            return Err(miette!("Search command failed"));
        }

        // Dummy return - parsing logic would go here to extract name, version, etc.
        Ok(vec![
            Package {
                name: query.to_string(),
                version: "1.0.0".to_string(),
                description: "Simulated package description".to_string(),
                is_installed: false,
                repository: "core".to_string(),
            }
        ])
    }

    fn install(&self, packages: &[String]) -> Result<()> {
        let mut cmd = Command::new(&self.helper);
        
        // If not using an AUR helper, we need sudo for pacman installs
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }

        cmd.arg("-S").arg("--needed");
        for pkg in packages {
            cmd.arg(pkg);
        }

        let status = cmd.status().map_err(|e| miette!("Failed to execute install: {}", e))?;
        if !status.success() {
            return Err(miette!("Install command failed"));
        }

        Ok(())
    }

    fn remove(&self, packages: &[String]) -> Result<()> {
        let mut cmd = Command::new(&self.helper);
        
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }

        cmd.arg("-Rs"); // Remove package and unneeded dependencies
        for pkg in packages {
            cmd.arg(pkg);
        }

        let status = cmd.status().map_err(|e| miette!("Failed to execute remove: {}", e))?;
        if !status.success() {
            return Err(miette!("Remove command failed"));
        }

        Ok(())
    }

    fn update(&self) -> Result<()> {
        let mut cmd = Command::new(&self.helper);
        
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }

        cmd.arg("-Syu");

        let status = cmd.status().map_err(|e| miette!("Failed to execute update: {}", e))?;
        if !status.success() {
            return Err(miette!("Update command failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        // Remove orphans
        // pacman -Qtdq | sudo pacman -Rns -
        println!("Cleaning orphans and package cache...");
        
        // Dummy implementation
        Ok(())
    }
}
