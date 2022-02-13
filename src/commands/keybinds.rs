use crate::keybind::{get_bound_keys, CtrlgKeybind};
use clap::Args;
use skim::prelude::Key;
use tabled::{Alignment, Full, MaxWidth, Modify, Row, Style, Table, Tabled};

#[derive(Debug, Args)]
pub struct Cmd {}

#[derive(Tabled)]
struct KeybindEntry {
    key: String,
    description: String,
}

impl From<&Key> for KeybindEntry {
    fn from(key: &Key) -> Self {
        KeybindEntry {
            key: key.key_code().to_string(),
            description: key.description().to_string(),
        }
    }
}

impl Cmd {
    pub fn run(&self) -> Result<String, ()> {
        let keybinds = get_bound_keys()
            .iter()
            .map(KeybindEntry::from)
            .collect::<Vec<KeybindEntry>>();
        Ok(format!(
            "{}\n\n{}",
            include_str!("./ascii_logo.txt"),
            Table::new(keybinds)
                .with(Style::modern())
                .with(Modify::new(Full).with(Alignment::left()))
                .with(Modify::new(Row(1..)).with(MaxWidth::wrapping(50).keep_words()))
        ))
    }
}
