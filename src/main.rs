use crate::{dirs::get_dirs, settings::Settings};
use std::error::Error;

mod dirs;
mod finder;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new()?;
    let dirs = get_dirs(&settings)?;
    finder::find(&dirs, settings.preview == true);
    Ok(())
}
