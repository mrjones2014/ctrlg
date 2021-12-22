use crate::command_strs;
use crate::dir_item::DirItem;
use crate::settings::{Settings, SETTINGS};
use skim::prelude::*;
use skim::{prelude::unbounded, SkimItem, SkimItemReceiver, SkimItemSender};
use std::{borrow::Cow, path::PathBuf, sync::Arc};

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
            if Settings::get_readonly().preview_fallback_exa {
                return ItemPreview::Command(format!(
                    "{} \"{}\"",
                    command_strs::EXA.join(" "),
                    self.path.to_str().unwrap().to_string()
                ));
            }

            return ItemPreview::Command(format!(
                "{} \"{}\"",
                command_strs::LS.join(" "),
                self.path.to_str().unwrap().to_string()
            ));
        }

        let readme_path = self.readme.as_ref().unwrap();
        if SETTINGS.lock().unwrap().preview_with_bat {
            ItemPreview::Command(format!(
                "{} \"{}\"",
                command_strs::BAT.join(" "),
                readme_path.to_str().unwrap().to_string()
            ))
        } else {
            ItemPreview::Command(format!(
                "{} \"{}\"",
                command_strs::CAT.join(" "),
                readme_path.to_str().unwrap().to_string()
            ))
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
        .preview(if Settings::get_readonly().preview {
            Some("")
        } else {
            None
        })
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
                return Some(selected.path.to_str().unwrap().to_string());
            }

            None
        })
        .flatten()
}
