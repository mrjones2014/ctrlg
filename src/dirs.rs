use crate::{
    dir_item::{DirItem, DirItemError},
    settings::Settings,
};
use glob::{glob, GlobError};
use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum GetDirsError {
    DirItemError(DirItemError),
    GlobError(GlobError),
    IoError(io::Error),
}

impl Error for GetDirsError {}

impl From<DirItemError> for GetDirsError {
    fn from(e: DirItemError) -> Self {
        GetDirsError::DirItemError(e)
    }
}

impl From<GlobError> for GetDirsError {
    fn from(e: GlobError) -> Self {
        GetDirsError::GlobError(e)
    }
}

impl From<io::Error> for GetDirsError {
    fn from(e: io::Error) -> Self {
        GetDirsError::IoError(e)
    }
}

impl Display for GetDirsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetDirsError::DirItemError(e) => writeln!(f, "Error parsing directory metadata: {}", e),
            GetDirsError::GlobError(e) => writeln!(f, "Error expanding globbing pattern: {}", e),
            GetDirsError::IoError(e) => writeln!(f, "I/O error: {}", e),
        }
    }
}

pub fn get_dirs() -> Result<Vec<DirItem>, GetDirsError> {
    let mut items = Vec::new();
    for dir in Settings::global().search_dirs.iter() {
        let dir = shellexpand::tilde(dir);
        for child in glob(&dir).expect("Failed to resolve globbing pattern") {
            let mut path = child?;
            if path.is_dir() {
                items.push(DirItem::new(path)?);
            } else if !&dir.ends_with('*') {
                // globbing pattern is to a file like `~/git/**/package.json`
                path.pop();
                if path.is_dir() {
                    items.push(DirItem::new(path)?);
                }
            }
        }
    }

    items.sort_unstable_by_key(|item| item.display.to_string());

    Ok(items)
}
