use glob::{glob, GlobError};

use crate::settings::Settings;
use std::{error::Error, fmt::Display, fs, io, path::PathBuf};

#[derive(Debug, Clone)]
pub struct DirItem {
    pub path: String,
    pub readme: Option<String>,
}

fn get_readme(path: PathBuf) -> Result<Option<String>, io::Error> {
    let files = fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let os_file_name = file.file_name();
        let name = os_file_name.to_str();
        if name.is_none() {
            continue;
        }

        if name.unwrap().starts_with("README") {
            return Ok(Some(name.unwrap().to_string()));
        }
    }

    Ok(None)
}

#[derive(Debug)]
pub enum GetDirsError {
    IoError(io::Error),
    GlobError(GlobError),
}

impl Error for GetDirsError {}

impl From<io::Error> for GetDirsError {
    fn from(e: io::Error) -> Self {
        GetDirsError::IoError(e)
    }
}

impl From<GlobError> for GetDirsError {
    fn from(e: GlobError) -> Self {
        GetDirsError::GlobError(e)
    }
}

impl Display for GetDirsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetDirsError::IoError(e) => writeln!(f, "I/O Error: {}", e),
            GetDirsError::GlobError(e) => writeln!(f, "Error expanding globbing pattern: {}", e),
        }
    }
}

pub fn get_dirs(settings: &Settings) -> Result<Vec<DirItem>, GetDirsError> {
    let mut items = Vec::new();
    for dir in settings.search_dirs.iter() {
        let dir = shellexpand::tilde(dir);
        for child in glob(&dir).expect("Failed to resolve globbing pattern") {
            let path = child?;
            if path.is_dir() {
                items.push(DirItem {
                    path: path.to_str().unwrap().to_string(),
                    readme: get_readme(path)?,
                });
            }
        }
    }

    Ok(items)
}
