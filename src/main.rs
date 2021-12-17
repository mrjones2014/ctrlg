use crate::{dirs::get_dirs, settings::Settings};
use std::error::Error;

mod dirs;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new()?;
    println!("{:?}", get_dirs(settings)?);
    Ok(())
}
