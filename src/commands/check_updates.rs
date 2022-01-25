use clap::Args;
use serde_json::Value;
use std::error::Error;

use crate::version::Version;

#[derive(Debug, Args)]
#[clap(about = "Check if there are updates available for ctrlg")]
pub struct Cmd {}

impl Cmd {
    pub fn run(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let response: Value = client
            .get("https://github.com/mrjones2014/ctrlg/releases/latest")
            .header("Accept", "application/json")
            .send()?
            .json()?;
        let version_field = &response["tag_name"]
            .to_string()
            .replace('v', "")
            .replace('"', "");
        let current_version = Version::try_from(env!("CARGO_PKG_VERSION"))?;
        let latest_version = Version::try_from(version_field.to_string())?;
        if current_version >= latest_version {
            return Ok(format!("You're on the latest version! {}", current_version));
        }

        let instructions = format!(
            "A new version ({}) is available!\n\n{}\n\n{}",
            latest_version,
            include_str!("./ascii_logo.txt"),
            include_str!("./update_instructions.txt")
        );
        Ok(instructions)
    }
}
