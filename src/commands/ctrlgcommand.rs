use super::init;
use crate::{dirs::get_dirs, finder::find};
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum CtrlgCommand {
    #[structopt(about = "Find a directory based on configured globbing patterns")]
    Find,
    #[structopt(about = "Set up ctrl+g keybind for specified shell")]
    Init(init::Cmd),
}

impl CtrlgCommand {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            CtrlgCommand::Find => {
                let dirs = get_dirs()?;
                let selected = find(&dirs);
                if let Some(selected) = selected {
                    println!("{}", selected);
                }
                Ok(())
            }
            CtrlgCommand::Init(cmd) => {
                cmd.run();
                Ok(())
            }
        }
    }
}
