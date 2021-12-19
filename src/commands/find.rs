use structopt::StructOpt;

use crate::{
    dirs::{get_dirs, GetDirsError},
    finder::find,
    settings::Settings,
};

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
        let selected = find(&dirs);
        Ok(selected)
    }
}
