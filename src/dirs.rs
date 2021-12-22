use crate::{dir_item::DirItem, settings::Settings};
use glob::{glob, GlobError};
use std::{error::Error, fmt::Display, io};

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

pub fn get_dirs() -> Result<Vec<DirItem>, GetDirsError> {
    let mut items = Vec::new();
    for dir in Settings::get_readonly().search_dirs.iter() {
        let dir = shellexpand::tilde(dir);
        for child in glob(&dir).expect("Failed to resolve globbing pattern") {
            let path = child?;
            if path.is_dir() {
                items.push(DirItem::new(path)?);
            }
        }
    }

    items.sort_unstable_by_key(|item| item.display.to_string());

    Ok(items)
}
