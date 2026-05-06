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

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();
        let mut current_package: Option<Package> = None;

        for line in stdout.lines() {
            let line = line.trim_end();
            if line.is_empty() { continue; }
            
            if !line.starts_with(' ') {
                // This is a package header line: `repo/name version (groups) [installed]`
                if let Some(pkg) = current_package.take() {
                    packages.push(pkg);
                }
                
                let mut parts = line.split_whitespace();
                let repo_name = parts.next().unwrap_or("");
                let version = parts.next().unwrap_or("").to_string();
                
                let (repo, name) = if let Some((r, n)) = repo_name.split_once('/') {
                    (r, n)
                } else {
                    ("", repo_name)
                };
                
                let lower_line = line.to_lowercase();
                let is_installed = lower_line.contains("installed") || lower_line.contains("installiert");
                
                current_package = Some(Package {
                    name: name.to_string(),
                    version,
                    description: String::new(),
                    is_installed,
                    repository: repo.to_string(),
                });
            } else {
                // Description line (indented)
                if let Some(mut pkg) = current_package.take() {
                    pkg.description = line.trim().to_string();
                    packages.push(pkg);
                }
            }
        }
        
        if let Some(pkg) = current_package {
            packages.push(pkg);
        }

        Ok(packages)
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
