use config::{Config, ConfigError, File};
use dirs_next::home_dir;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};

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

fn user_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut base_paths = Vec::new();

    if let Some(home) = home_dir() {
        let home_config_path = [
            home.to_str().expect("Failed to expand $HOME"),
            ".config",
            "ctrlg",
        ]
        .iter()
        .collect::<PathBuf>();
        base_paths.push(home_config_path.to_str().unwrap().to_string());
    }

    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        let xdg_config_home_path = [xdg_config_home, String::from("ctrlg")]
            .iter()
            .collect::<PathBuf>();
        base_paths.push(xdg_config_home_path.to_str().unwrap().to_string());
    }

    for base_path in base_paths.iter() {
        for config_file_name in CONFIG_FILE_NAMES.iter() {
            let path = [base_path, *config_file_name].iter().collect::<PathBuf>();
            paths.push(path);
        }
    }

    paths
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
        for user_config_path in user_config_paths().iter() {
            if user_config_path.exists() {
                s.merge(File::with_name(user_config_path.to_str().unwrap()))?;
                break;
            }
        }

        s.try_into()
    }

    pub fn global() -> Self {
        let settings = SETTINGS.get().expect("Settings not initialized.");
        settings.to_owned()
    }

    pub fn init() -> Result<Self, ConfigError> {
        let settings = Settings::new()?;
        SETTINGS.set(settings.clone()).unwrap();
        Ok(settings)
    }
}

pub static SETTINGS: OnceCell<Settings> = OnceCell::new();
