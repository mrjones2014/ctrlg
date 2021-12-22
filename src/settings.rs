use crate::commands::find;
use config::{Config, ConfigError, File};
use dirs_next::home_dir;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, sync::Mutex};

const CONFIG_FILE_NAMES: [&str; 2] = ["config.yml", "config.yaml"];

#[derive(Debug, Deserialize, Clone)]
pub struct ColorSettings {
    pub dir_name: String,
    pub git_branch: String,
    pub bat_theme: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub search_dirs: Vec<String>,
    pub preview_files: Vec<String>,
    pub preview: bool,
    pub preview_with_bat: bool,
    pub preview_fallback_exa: bool,
    pub show_git_branch: bool,
    pub git_branch_separator: String,
    pub colors: ColorSettings,
}

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.set_default("search_dirs", vec!["~/git/*"])?;
        s.set_default("preview_files", vec!["README.*"])?;
        s.set_default("preview", true)?;
        s.set_default("preview_with_bat", is_program_in_path("bat"))?;
        s.set_default("preview_fallback_exa", is_program_in_path("exa"))?;
        s.set_default("show_git_branch", true)?;
        s.set_default("git_branch_separator", "■")?;
        s.set_default("colors.dir_name", "cyan")?;
        s.set_default("colors.git_branch", "247,78,39")?; // git brand orange color
        s.set_default("colors.bat_theme", "ansi")?;

        let home = home_dir();
        if home.is_none() {
            return s.try_into();
        }

        // merge user config if it exists
        let home = home.unwrap();
        for config_file_name in CONFIG_FILE_NAMES.iter() {
            let user_config_path: PathBuf = [
                home.to_str().expect("Failed to determine config directory"),
                ".config",
                "ctrlg",
                config_file_name,
            ]
            .iter()
            .collect();

            if user_config_path.exists() {
                s.merge(File::with_name(user_config_path.to_str().unwrap()))?;
                break;
            }
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
    pub static ref SETTINGS: Mutex<Settings> = {
        let settings = Settings::new();
        if let Err(e) = settings {
            panic!("Failed to parse yml from configuration file: {}", e);
        }

        Mutex::from(settings.unwrap())
    };
}
