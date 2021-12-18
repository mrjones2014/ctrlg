use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cmd {
    #[structopt(about = "Set up ctrl+g keybind for Fish shell")]
    Fish,
    #[structopt(about = "Set up ctrl+g keybind for Bash")]
    Bash,
}

impl Cmd {
    pub fn run(&self) {
        match self {
            Cmd::Fish => {
                let script = include_str!("./shell/ctrlg.fish");
                println!("{}", script);
            }
            Cmd::Bash => {
                let script = include_str!("./shell/ctrlg.bash");
                println!("{}", script);
            }
        }
    }
}
