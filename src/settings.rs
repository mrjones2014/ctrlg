use config::{Config, ConfigError, File};
use dirs_next::home_dir;
use serde::Deserialize;
use std::{path::PathBuf, sync::Mutex};

use crate::commands::find;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub search_dirs: Vec<String>,
    pub preview_files: Vec<String>,
    pub preview: bool,
    pub preview_with_bat: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.set_default("search_dirs", vec!["~/git/*"])?;
        s.set_default("preview_files", vec!["README.*"])?;
        s.set_default("preview", true)?;
        s.set_default("preview_with_bat", false)?;

        let home = home_dir();
        if home.is_none() {
            return s.try_into();
        }

        // merge user config if it exists
        let home = home.unwrap();
        let user_config_path: PathBuf = [
            home.to_str().expect("Failed to determine config directory"),
            ".config",
            "ctrlg",
            "config.yml",
        ]
        .iter()
        .collect();
        if user_config_path.exists() {
            s.merge(File::with_name(user_config_path.to_str().unwrap()))?;
        }

        s.try_into()
    }

    pub fn merge_find_args(find_args: &find::Cmd) {
        let mut settings_mut = SETTINGS.lock().unwrap();

        if let Some(search_dirs) = &find_args.search_dirs {
            settings_mut.search_dirs = search_dirs.clone();
        }

        if let Some(preview_files) = &find_args.preview_files {
            settings_mut.preview_files = preview_files.clone();
        }

        if let Some(preview) = find_args.preview {
            settings_mut.preview = preview;
        }

        if let Some(preview_with_bat) = find_args.preview_with_bat {
            settings_mut.preview_with_bat = preview_with_bat;
        }
    }

    pub fn get_readonly() -> Self {
        let settings_lock = SETTINGS.lock().unwrap();
        let settings = settings_lock.clone();
        drop(settings_lock);
        settings
    }
}

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::from(Settings::new().unwrap());
}
