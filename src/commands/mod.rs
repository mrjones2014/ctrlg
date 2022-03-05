use clap::{AppSettings, Subcommand};
use std::error::Error;

pub mod check_updates;
pub mod find;
pub mod init;
mod keybinds;

#[derive(Debug, Subcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder, propagate_version = true)]
pub enum CtrlgCommand {
    #[clap(about = "Find a directory based on configured globbing patterns")]
    Find(find::Cmd),
    #[clap(subcommand)]
    Init(init::Cmd),
    #[clap(about = "Check if updates are available for ctrlg")]
    CheckUpdates(check_updates::Cmd),
    #[clap(about = "Print the key bindings used for the fuzzy finder")]
    Keybinds(keybinds::Cmd),
}

impl CtrlgCommand {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            CtrlgCommand::Find(cmd) => {
                let selected = cmd.run()?;
                if let Some(selected) = selected {
                    println!("{}", selected);
                }
                Ok(())
            }
            CtrlgCommand::Init(cmd) => {
                let script = cmd.run();
                println!("{}", script);
                Ok(())
            }
            CtrlgCommand::CheckUpdates(cmd) => {
                let instructions = cmd.run()?;
                println!("{}", instructions);
                Ok(())
            }
            CtrlgCommand::Keybinds(cmd) => {
                let keybinds_table = cmd.run().unwrap();
                println!("{}", keybinds_table);
                Ok(())
            }
        }
    }
}
