use clap::{AppSettings, Parser};
use commands::CtrlgCommand;
use settings::Settings;
use std::error::Error;

mod colors;
mod command_strs;
mod commands;
mod dir_item;
mod dirs;
mod finder;
mod git_meta;
mod settings;

#[derive(Debug, Parser)]
#[clap(
    author = "Mat Jones <mat@mjones.network>",
    version = env!("CARGO_PKG_VERSION"),
    about = include_str!("./cli_about.txt"),
    global_setting = AppSettings::PropagateVersion,
    global_setting = AppSettings::DeriveDisplayOrder,
)]
struct Ctrlg {
    #[clap(subcommand)]
    ctrlg: CtrlgCommand,
}

impl Ctrlg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        self.ctrlg.run()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Settings::init()?;
    Ctrlg::parse().run()?;
    Ok(())
}
