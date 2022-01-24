use clap::{AppSettings, Subcommand};
use std::error::Error;

pub mod check_updates;
pub mod find;
pub mod init;

#[derive(Debug, Subcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
pub enum CtrlgCommand {
    #[clap(about = "Find a directory based on configured globbing patterns")]
    Find(find::Cmd),
    #[clap(subcommand)]
    Init(init::Cmd),
    CheckUpdates(check_updates::Cmd),
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
        }
    }
}
