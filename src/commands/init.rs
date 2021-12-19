use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cmd {
    #[structopt(about = "Set up ctrl+g keybind for Fish shell")]
    Fish,
    #[structopt(about = "Set up ctrl+g keybind for Bash")]
    Bash,
    #[structopt(about = "Set up ctrl+g keybind for Zsh")]
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
