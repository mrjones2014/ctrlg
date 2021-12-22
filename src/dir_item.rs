use crate::{command_strs, settings::Settings};
use glob::glob;
use std::{fmt::Display, io, path::PathBuf, process::Command, string::FromUtf8Error};

#[derive(Debug)]
pub enum DirItemError {
    IO(io::Error),
    UTF8(FromUtf8Error),
}

impl From<io::Error> for DirItemError {
    fn from(e: io::Error) -> Self {
        DirItemError::IO(e)
    }
}

impl From<FromUtf8Error> for DirItemError {
    fn from(e: FromUtf8Error) -> Self {
        DirItemError::UTF8(e)
    }
}

impl Display for DirItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirItemError::IO(e) => write!(f, "Error reading directory: {}", e),
            DirItemError::UTF8(e) => write!(
                f,
                "Error parsing {} output: {}",
                command_strs::DIR_INFO_HOOK[0],
                e
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirItem {
    pub path: PathBuf,
    pub display: String,
    pub readme: Option<PathBuf>,
}

impl DirItem {
    pub fn new(path: PathBuf) -> Result<Self, DirItemError> {
        let display = get_display(&path)?;
        let readme = get_readme(&path)?;

        Ok(Self {
            path,
            display,
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

    let result = Command::new(command_strs::DIR_INFO_HOOK[0])
        .arg(path.to_str().expect("Failed to expand path").to_string())
        .output();

    if let Ok(output) = result {
        let info = String::from_utf8(output.stdout)?;
        println!("{}", info);
        let first_line = info.lines().next();
        if let Some(line) = first_line {
            display = format!(
                "{} {} {}",
                display,
                Settings::get_readonly().dir_info_separator,
                line
            );
        }
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
