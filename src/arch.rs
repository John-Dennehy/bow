use crate::provider::{Package, PackageProvider};
use miette::{miette, Result};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use inquire::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement};
use owo_colors::OwoColorize;

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
        println!("{} Packages to install:", "=>".blue().bold());
        for pkg in packages {
            println!("   {} {}", "•".magenta(), pkg.white());
        }
        
        let ans = Confirm::new("Proceed with installation?")
            .with_default(true)
            .prompt()
            .unwrap_or(false);
            
        if !ans {
            println!("{} Installation cancelled.", "=>".red().bold());
            return Ok(());
        }

        let mut cmd = Command::new(&self.helper);
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }

        cmd.arg("-S").arg("--needed").arg("--noconfirm");
        for pkg in packages {
            cmd.arg(pkg);
        }

        run_with_spinner(cmd, "Installing packages...", "Installation complete!")?;
        Ok(())
    }

    fn remove(&self, packages: &[String]) -> Result<()> {
        println!("{} Packages to remove:", "=>".yellow().bold());
        for pkg in packages {
            println!("   {} {}", "•".magenta(), pkg.white());
        }
        
        let ans = Confirm::new("Proceed with removal?")
            .with_default(true)
            .prompt()
            .unwrap_or(false);
            
        if !ans {
            println!("{} Removal cancelled.", "=>".red().bold());
            return Ok(());
        }

        let mut cmd = Command::new(&self.helper);
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }

        cmd.arg("-Rs").arg("--noconfirm");
        for pkg in packages {
            cmd.arg(pkg);
        }

        run_with_spinner(cmd, "Removing packages...", "Removal complete!")?;
        Ok(())
    }

    fn update(&self) -> Result<()> {
        println!("{} Checking for updates...", "=>".blue().bold());
        let output = Command::new(&self.helper)
            .arg("-Qu")
            .output()
            .map_err(|e| miette!("Failed to check for updates: {}", e))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
        
        if lines.is_empty() {
            println!("{} System is already up to date!", "=>".green().bold());
            return Ok(());
        }
        
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL)
             .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
             .set_content_arrangement(ContentArrangement::Dynamic)
             .set_header(vec!["Package", "Old Version", "New Version"]);
             
        for line in &lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Usually yay -Qu is: name old -> new
            if parts.len() >= 4 && parts[parts.len() - 2] == "->" {
                let new_ver = parts.last().unwrap();
                let old_ver = parts[parts.len() - 3];
                let name = parts[0..parts.len() - 3].join(" ");
                
                table.add_row(vec![
                    Cell::new(name).add_attribute(Attribute::Bold),
                    Cell::new(old_ver).fg(Color::DarkGrey),
                    Cell::new(new_ver).fg(Color::Green),
                ]);
            } else {
                table.add_row(vec![line.to_string(), "".to_string(), "".to_string()]);
            }
        }
        
        println!("{table}");
        println!("{} {} packages to update.", "=>".yellow().bold(), lines.len());
        
        let ans = Confirm::new("Proceed with update?")
            .with_default(true)
            .prompt()
            .unwrap_or(false);
            
        if !ans {
            println!("{} Update cancelled.", "=>".red().bold());
            return Ok(());
        }
        
        let mut cmd = Command::new(&self.helper);
        if self.helper == "pacman" {
            cmd = Command::new("sudo");
            cmd.arg("pacman");
        }
        cmd.arg("-Syu").arg("--noconfirm");

        run_with_spinner(cmd, "Updating system...", "Update complete!")?;
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        println!("{} Scanning for orphaned packages...", "=>".blue().bold());
        
        // Query for orphans: -Q (query), -t (unrequired dependencies), -d (deps), -q (quiet/names only)
        let output = Command::new("pacman")
            .arg("-Qtdq")
            .output()
            .map_err(|e| miette!("Failed to check for orphans: {}", e))?;
            
        // pacman -Qtdq returns an error code if no orphans are found, or an empty stdout
        if !output.status.success() || output.stdout.is_empty() {
            println!("{} No orphaned packages found. System is clean!", "=>".green().bold());
            return Ok(());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let orphans: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
        
        if orphans.is_empty() {
            println!("{} No orphaned packages found. System is clean!", "=>".green().bold());
            return Ok(());
        }

        println!("{} Found {} orphaned packages to remove:", "=>".yellow().bold(), orphans.len());
        for pkg in &orphans {
            println!("   {} {}", "•".magenta(), pkg.white());
        }

        let ans = Confirm::new("Proceed with cleanup?")
            .with_default(true)
            .prompt()
            .unwrap_or(false);
            
        if !ans {
            println!("{} Cleanup cancelled.", "=>".red().bold());
            return Ok(());
        }

        let mut cmd = Command::new("sudo");
        cmd.arg("pacman").arg("-Rns").arg("--noconfirm");
        for pkg in orphans {
            cmd.arg(pkg);
        }

        run_with_spinner(cmd, "Pruning orphaned packages...", "Cleanup complete!")?;
        Ok(())
    }
}

fn run_with_spinner(mut cmd: Command, msg: &str, success_msg: &str) -> Result<()> {
    // Elevate privileges natively BEFORE we hide output and enter raw mode.
    // This prevents sudo from prompting for a password invisibly and stealing tty.
    let _ = Command::new("sudo").arg("-v").status();

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| miette!("Failed to start command: {}", e))?;
    let stdout_pipe = child.stdout.take().unwrap();
    let stderr_pipe = child.stderr.take().unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    
    let tx_out = tx.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout_pipe);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = tx_out.send(l);
            }
        }
    });

    let tx_err = tx.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stderr_pipe);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = tx_err.send(l);
            }
        }
    });

    enable_raw_mode().unwrap_or(());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message(format!("{} Press 'v' for raw output", msg));
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut show_raw = false;
    
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            pb.finish_and_clear();
            disable_raw_mode().unwrap_or(());
            if status.success() {
                println!("{} {}", "=>".green().bold(), success_msg);
            } else {
                return Err(miette!("Command failed with status: {}", status));
            }
            break;
        }

        if event::poll(Duration::from_millis(50)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.code == KeyCode::Char('v') && !show_raw {
                    show_raw = true;
                    pb.finish_and_clear();
                    disable_raw_mode().unwrap_or(());
                    println!("{} Switching to raw output...", "=>".cyan().bold());
                } else if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    pb.finish_and_clear();
                    disable_raw_mode().unwrap_or(());
                    let _ = child.kill();
                    return Err(miette!("Aborted by user."));
                }
            }
        }

        while let Ok(msg) = rx.try_recv() {
            if show_raw {
                println!("{}", msg);
            }
        }
    }

    Ok(())
}
