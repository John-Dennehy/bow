use directories::ProjectDirs;
use miette::{miette, Result};
use std::path::PathBuf;

pub struct Config {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub state_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "bow", "bow")
            .ok_or_else(|| miette!("Could not determine XDG base directories for your system."))?;

        let config = Self {
            config_dir: proj_dirs.config_dir().to_path_buf(),
            cache_dir: proj_dirs.cache_dir().to_path_buf(),
            state_dir: proj_dirs.state_dir().unwrap_or(proj_dirs.data_local_dir()).to_path_buf(),
        };

        // Create directories if they don't exist
        std::fs::create_dir_all(&config.config_dir).map_err(|e| miette!("Failed to create config directory: {}", e))?;
        std::fs::create_dir_all(&config.cache_dir).map_err(|e| miette!("Failed to create cache directory: {}", e))?;
        std::fs::create_dir_all(&config.state_dir).map_err(|e| miette!("Failed to create state directory: {}", e))?;

        Ok(config)
    }
}
