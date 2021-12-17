use std::path::PathBuf;

use config::{Config, ConfigError, File};
use dirs_next::home_dir;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub search_dirs: Vec<String>,
    pub preview: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.set_default("search_dirs", vec!["~/git"])?;
        s.set_default("preview", true)?;

        let home = home_dir();
        if home.is_none() {
            return s.try_into();
        }

        // merge user config if it exists
        let home = home.unwrap();
        let user_config_path: PathBuf = [
            home.to_str().expect("Failed to determine config directory"),
            ".config",
            "dirswitch",
            "config.yml",
        ]
        .iter()
        .collect();
        if user_config_path.exists() {
            s.merge(File::with_name(user_config_path.to_str().unwrap()))?;
        }

        s.try_into()
    }
}
