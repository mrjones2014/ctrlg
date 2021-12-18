use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cmd {
    #[structopt(about = "Set up ctrl+k keybind for Fish shell")]
    Fish,
}

impl Cmd {
    pub fn run(&self) {
        match self {
            Cmd::Fish => {
                let script = include_str!("./shell/ctrlk.fish");
                println!("{}", script);
            }
        }
    }
}
