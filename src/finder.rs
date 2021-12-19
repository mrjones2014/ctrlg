use crate::dirs::DirItem;
use crate::settings::SETTINGS;
use skim::prelude::*;
use skim::{prelude::unbounded, SkimItem, SkimItemReceiver, SkimItemSender};
use std::{borrow::Cow, path::PathBuf, sync::Arc};
use std::{env, fs};

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

impl SkimItem for DirItem {
    fn text(&self) -> std::borrow::Cow<str> {
        let path = PathBuf::from(self.path.clone());
        let dir_name = path.file_name();
        if dir_name.is_none() {
            return Cow::from("");
        }

        let dir_name = dir_name.unwrap();
        let dir_name = dir_name.to_str();
        if dir_name.is_none() {
            return Cow::from("");
        }

        let dir_name = dir_name.unwrap();
        Cow::from(dir_name.to_string())
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        if self.readme.is_none() {
            return ItemPreview::Command(format!("ls {}", self.path));
        }

        if self.readme.is_none() {
            return ItemPreview::Command(format!("ls {}", self.path));
        }
        let readme_path = self.readme.as_ref().unwrap();
        if SETTINGS.preview_with_bat {
            ItemPreview::Command(format!("bat --style=plain --color=always {}", readme_path))
        } else {
            ItemPreview::Command(format!("cat {}", readme_path))
        }
    }
}

fn receiver(items: &[DirItem]) -> SkimItemReceiver {
    let (tx_items, rx_items): (SkimItemSender, SkimItemReceiver) = unbounded();
    items.iter().for_each(|feature| {
        let _ = tx_items.send(Arc::new(feature.to_owned()));
    });
    drop(tx_items); // indicates that all items have been sent
    rx_items
}

pub fn find(items: &[DirItem]) -> Option<String> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .preview(if SETTINGS.preview { Some("") } else { None })
        .multi(false)
        .build()
        .unwrap();

    let receiver = receiver(items);

    Skim::run_with(&skim_options, Some(receiver))
        .map(|out| {
            let selected = out.selected_items.first();
            let selected = match selected {
                Some(item) => {
                    let selected_dir = (**item).as_any().downcast_ref::<DirItem>().unwrap();
                    Some(selected_dir)
                }
                None => None,
            };

            selected?;

            let selected = selected.unwrap();

            if out.final_key == Key::Enter {
                return Some(selected.path.to_string());
            }

            None
        })
        .flatten()
}
