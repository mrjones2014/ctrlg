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
    pub preview_with_glow: bool,
    pub glow_wrap_width: usize,
    pub preview_fallback_exa: bool,
    pub show_git_branch: bool,
    pub git_branch_separator: String,
    pub colors: ColorSettings,
    pub include: Vec<String>,
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

fn normalize_path_if_exists(path: String) -> Option<String> {
    let file_path = shellexpand::tilde(&path).to_string();
    if PathBuf::from(&file_path).is_file() {
        return Some(file_path);
    }

    None
}

// custom merge so that we combine Vecs instead of replacing them
fn merge_include(config: &mut Config, included: &Config) -> Result<(), ConfigError> {
    let mut search_dirs = config.get_array("search_dirs").unwrap_or_default();
    let mut included_seach_dirs = included.get_array("search_dirs").unwrap_or_default();
    included_seach_dirs.append(&mut search_dirs);
    config.set("search_dirs", included_seach_dirs)?;

    let mut preview_files = config.get_array("preview_files").unwrap_or_default();
    let mut included_preview_files = included.get_array("preview_files").unwrap_or_default();
    included_preview_files.append(&mut preview_files);
    config.set("preview_files", included_preview_files)?;

    if let Ok(preview) = included.get_bool("preview") {
        config.set("preview", preview)?;
    }

    if let Ok(preview_with_bat) = included.get_bool("preview_with_bat") {
        config.set("preview_with_bat", preview_with_bat)?;
    }

    if let Ok(preview_fallback_exa) = included.get_bool("preview_fallback_exa") {
        config.set("preview_fallback_exa", preview_fallback_exa)?;
    }

    if let Ok(show_git_branch) = included.get_bool("show_git_branch") {
        config.set("show_git_branch", show_git_branch)?;
    }

    if let Ok(colors) = included.get_table("colors") {
        config.set("colors", colors)?;
    }

    Ok(())
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        let glow_installed = is_program_in_path("glow");
        let bat_installed = is_program_in_path("bat");
        let exa_installed = is_program_in_path("exa");

        s.set_default("search_dirs", vec!["~/git/*"])?;
        s.set_default("preview_files", vec!["README.*"])?;
        s.set_default("preview", true)?;
        s.set_default("preview_with_glow", glow_installed)?;
        s.set_default("glow_wrap_width", 80)?;
        s.set_default("preview_with_bat", !glow_installed && bat_installed)?;
        s.set_default("preview_fallback_exa", exa_installed)?;
        s.set_default("show_git_branch", true)?;
        s.set_default("git_branch_separator", "■")?;
        s.set_default("colors.dir_name", "cyan")?;
        s.set_default("colors.git_branch", "247,78,39")?; // git brand orange color
        s.set_default("colors.bat_theme", "ansi")?;
        s.set_default("include", Vec::<String>::new())?;

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

        let includes = s.get_array("include")?;
        for include_path in includes.iter() {
            let path_str = include_path.clone().into_str()?.to_string();
            let normalized = normalize_path_if_exists(path_str);
            if let Some(normalized) = normalized {
                let mut included_config = Config::default();
                included_config.merge(File::with_name(&normalized))?;
                merge_include(&mut s, &included_config)?;
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
