#[macro_use]
extern crate lazy_static;
use commands::CtrlgCommand;
use std::error::Error;
use structopt::{clap::AppSettings, StructOpt};

mod command_strs;
mod commands;
mod dir_item;
mod dirs;
mod finder;
mod settings;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(StructOpt)]
#[structopt(
    author = "Mat Jones <mat@mjones.network>",
    version = VERSION,
    about = "Press ctrl+g to search and jump to any directory",
    global_settings(&[AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder])
)]
struct Ctrlg {
    #[structopt(subcommand)]
    ctrlg: CtrlgCommand,
}

impl Ctrlg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        self.ctrlg.run()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Ctrlg::from_args().run()?;
    Ok(())
}
