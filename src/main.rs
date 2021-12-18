use commands::ctrlkcommand::CtrlkCommand;
use std::error::Error;
use structopt::{clap::AppSettings, StructOpt};

mod commands;
mod dirs;
mod finder;
mod settings;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(StructOpt)]
#[structopt(
    author = "Mat Jones <mat@mjones.network>",
    version = VERSION,
    about = "Press ctrl+k to search and jump to any directory",
    global_settings(&[AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder])
)]
struct Ctrlk {
    #[structopt(subcommand)]
    ctrlk: CtrlkCommand,
}

impl Ctrlk {
    fn run(self) -> Result<(), Box<dyn Error>> {
        self.ctrlk.run()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Ctrlk::from_args().run()?;
    Ok(())
}
