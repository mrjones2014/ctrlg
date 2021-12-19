use std::error::Error;
use structopt::StructOpt;

pub mod find;
pub mod init;

#[derive(StructOpt)]
pub enum CtrlgCommand {
    #[structopt(about = "Find a directory based on configured globbing patterns")]
    Find(find::Cmd),
    #[structopt(about = "Set up ctrl+g keybind for specified shell")]
    Init(init::Cmd),
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
        }
    }
}
