use crate::{git_meta, settings::Settings};
use ansi_term::Color::{Cyan, Red};
use glob::glob;
use std::{fmt::Display, io, path::PathBuf};

#[derive(Debug)]
pub enum DirItemError {
    IO(io::Error),
    Git(git2::Error),
}

impl From<io::Error> for DirItemError {
    fn from(e: io::Error) -> Self {
        DirItemError::IO(e)
    }
}

impl From<git2::Error> for DirItemError {
    fn from(e: git2::Error) -> Self {
        DirItemError::Git(e)
    }
}

impl Display for DirItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirItemError::IO(e) => write!(f, "Error reading directory: {}", e),
            DirItemError::Git(e) => write!(f, "Error reading git metadata: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirItem {
    pub path: PathBuf,
    pub display: String,
    pub match_str: String,
    pub readme: Option<PathBuf>,
}

impl DirItem {
    pub fn new(path: PathBuf) -> Result<Self, DirItemError> {
        let display = get_display(&path)?;
        let readme = get_readme(&path)?;
        let match_str = path
            .file_name()
            .expect("Failed to expand path")
            .to_str()
            .unwrap()
            .to_string();

        Ok(Self {
            path,
            display,
            match_str,
            readme,
        })
    }
}

fn get_display(path: &PathBuf) -> Result<String, DirItemError> {
    let mut display = path
        .file_name()
        .expect("Failed to expand path")
        .to_str()
        .unwrap()
        .to_string();

    if !Settings::get_readonly().show_git_branch {
        return Ok(display);
    }

    let branch = git_meta::get_current_branch(&path)?;
    if let Some(branch) = branch {
        display = format!(
            "{}  {} {}",
            Cyan.paint(display),
            Red.paint(Settings::get_readonly().git_branch_separator),
            Red.paint(branch)
        );
    }

    Ok(display)
}

fn get_readme(path: &PathBuf) -> Result<Option<PathBuf>, io::Error> {
    for glob_pattern in Settings::get_readonly().preview_files.iter() {
        let mut preview_file_pattern = path.clone();
        preview_file_pattern.push(glob_pattern);

        let preview_file_pattern = preview_file_pattern
            .to_str()
            .expect("Failed to expand preview file path");

        let matched_preview_file = glob(preview_file_pattern)
            .expect("Failed to expand preview file globbing pattern")
            .flatten()
            .next();

        if let Some(file) = matched_preview_file {
            return Ok(Some(file));
        }
    }

    Ok(None)
}
