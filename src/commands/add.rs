use std::env;

use crate::{dirs::GetDirsError, settings::Settings};
use clap::Args;

#[derive(Debug, Args)]
pub struct Cmd {
    dir: Option<String>,
}

impl Cmd {
    pub fn run(&self) -> Result<Option<String>, GetDirsError> {
        // get dir from argument or current dir if not provided
        let dir = shellexpand::tilde(
            &self
                .dir
                .unwrap_or(env::current_dir()?.to_string_lossy().to_string()),
        );

        let glob = format!("{}/*", dir).replace("//", "/");

        let mut settings = Settings::global();
        settings.search_dirs.push(glob);
        Settings::update(settings);
        Settings::write()?;
        Ok(Some(format!(
            "Added {} to search_dirs in config file.",
            glob
        )))
    }
}
