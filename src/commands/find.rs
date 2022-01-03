use crate::{
    dirs::{get_dirs, GetDirsError},
    finder::find,
    settings::Settings,
};
use clap::Args;

#[derive(Debug, Args)]
pub struct Cmd {}

impl Cmd {
    pub fn run(&self) -> Result<Option<String>, GetDirsError> {
        let dirs = get_dirs()?;
        if dirs.is_empty() {
            let search_dirs = Settings::global().search_dirs;
            let search_dirs_str = if search_dirs.is_empty() {
                String::from("[Empty list]")
            } else {
                Settings::global()
                    .search_dirs
                    .iter()
                    .map(|dir_str| format!("- {}", dir_str))
                    .collect::<Vec<String>>()
                    .join("\n")
            };
            eprintln!("No directories found under configured `search_dirs`, do you need to customize `search_dirs` in `~/.config/ctrlg/config.yml`?\nCurrent `search_dirs` value is configured as:\n{}\n\n", search_dirs_str);
            return Ok(None);
        }
        let selected = find(&dirs);
        Ok(selected)
    }
}
