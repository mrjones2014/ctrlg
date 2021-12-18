use super::init;
use crate::{dirs::get_dirs, finder::find, settings::Settings};
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum CtrlkCommand {
    #[structopt(about = "Find and go to a directory")]
    Go,
    #[structopt(about = "Set up ctrl+k keybind for specified shell")]
    Init(init::Cmd),
}

impl CtrlkCommand {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let settings = Settings::new()?;
        match self {
            CtrlkCommand::Go => {
                let dirs = get_dirs(&settings)?;
                let selected = find(&dirs, settings.preview);
                if let Some(selected) = selected {
                    println!("{}", selected);
                }
                Ok(())
            }
            CtrlkCommand::Init(cmd) => {
                cmd.run();
                Ok(())
            }
        }
    }
}
