use crate::settings::Settings;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct DirItem {
    pub path: String,
    pub readme: Option<String>,
}

fn get_readme(path: PathBuf) -> Result<Option<String>, std::io::Error> {
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

pub fn get_dirs(settings: Settings) -> Result<Vec<DirItem>, std::io::Error> {
    let mut items = Vec::new();
    for dir in settings.search_dirs.iter() {
        let dir = shellexpand::tilde(dir);
        let contents = fs::read_dir(dir.as_ref())?;
        for child in contents {
            let path = child?.path();
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
