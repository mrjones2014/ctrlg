use crate::settings::Settings;
use glob::glob;
use std::{io, path::PathBuf};

#[derive(Debug, Clone)]
pub struct DirItem {
    pub path: PathBuf,
    pub display: String,
    pub readme: Option<PathBuf>,
}

impl DirItem {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        let display = path
            .file_name()
            .expect("Failed to expand path")
            .to_str()
            .unwrap()
            .to_string();
        let readme = get_readme(&path)?;

        Ok(Self {
            path,
            display,
            readme,
        })
    }
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
