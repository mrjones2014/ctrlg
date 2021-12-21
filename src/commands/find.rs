use structopt::StructOpt;

use crate::{
    dirs::{get_dirs, GetDirsError},
    finder::find,
    settings::Settings,
};

// the global settings object
// gets these merged into it
// at runtime
#[derive(StructOpt)]
pub struct Cmd {
    #[structopt(long)]
    pub search_dirs: Option<Vec<String>>,
    #[structopt(long)]
    pub preview_files: Option<Vec<String>>,
    #[structopt(long)]
    pub preview: Option<bool>,
    #[structopt(long)]
    pub preview_with_bat: Option<bool>,
}

impl Cmd {
    pub fn run(&self) -> Result<Option<String>, GetDirsError> {
        Settings::merge_find_args(self);
        let dirs = get_dirs()?;
        if dirs.is_empty() {
            let search_dirs = Settings::get_readonly().search_dirs;
            let search_dirs_str = if search_dirs.is_empty() {
                String::from("[Empty list]")
            } else {
                Settings::get_readonly()
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
