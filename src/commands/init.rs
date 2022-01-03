use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[clap(about = "Set up ctrl+g keybind for specified shell")]
pub enum Cmd {
    #[clap(about = "Set up ctrl+g keybind for Fish shell")]
    Fish,
    #[clap(about = "Set up ctrl+g keybind for Bash")]
    Bash,
    #[clap(about = "Set up ctrl+g keybind for Zsh")]
    Zsh,
}

impl Cmd {
    pub fn run(&self) -> String {
        match self {
            Cmd::Fish => include_str!("./shell/ctrlg.fish"),
            Cmd::Bash => include_str!("./shell/ctrlg.bash"),
            Cmd::Zsh => include_str!("./shell/ctrlg.zsh"),
        }
        .to_string()
    }
}
