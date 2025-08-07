use anyhow::Result;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub behavior: Behavior,
    pub directories: Directories,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub accent_color: String,
    pub background_color: String,
    pub text_color: String,
    pub highlight_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub auto_sudo: bool,
    pub confirm_destructive_commands: bool,
    pub save_command_history: bool,
    pub max_output_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directories {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        let config_dir = home_dir.join(".config").join("linux-toolkit");
        let cache_dir = home_dir.join(".cache").join("linux-toolkit");
        let data_dir = home_dir.join(".local").join("share").join("linux-toolkit");

        Self {
            theme: Theme {
                accent_color: "Green".to_string(),
                background_color: "Black".to_string(),
                text_color: "White".to_string(),
                highlight_color: "Yellow".to_string(),
            },
            behavior: Behavior {
                auto_sudo: false,
                confirm_destructive_commands: true,
                save_command_history: true,
                max_output_lines: 1000,
            },
            directories: Directories {
                config_dir,
                cache_dir,
                data_dir,
            },
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&String>) -> Result<Self> {
        let config = if let Some(path) = config_path {
            let content = fs::read_to_string(path)?;
            toml::from_str(&content)?
        } else {
            let default_config = Self::default();
            let config_file = default_config.directories.config_dir.join("config.toml");

            if config_file.exists() {
                let content = fs::read_to_string(&config_file)?;
                toml::from_str(&content)?
            } else {
                // Create default config file
                default_config.save(None)?;
                default_config
            }
        };

        // Ensure directories exist
        fs::create_dir_all(&config.directories.config_dir)?;
        fs::create_dir_all(&config.directories.cache_dir)?;
        fs::create_dir_all(&config.directories.data_dir)?;

        Ok(config)
    }

    pub fn save(&self, config_path: Option<&String>) -> Result<()> {
        let path = if let Some(path) = config_path {
            PathBuf::from(path)
        } else {
            self.directories.config_dir.join("config.toml")
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.directories.config_dir.join("config.toml")
    }

    pub fn get_history_path(&self) -> PathBuf {
        self.directories.data_dir.join("command_history.json")
    }

    pub fn get_cache_path(&self) -> PathBuf {
        self.directories.cache_dir.join("cache.json")
    }
}
