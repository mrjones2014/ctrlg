use crate::command_strs;
use crate::dir_item::DirItem;
use crate::settings::{Settings, SETTINGS};
use skim::prelude::*;
use skim::{prelude::unbounded, SkimItem, SkimItemReceiver, SkimItemSender};
use std::{borrow::Cow, sync::Arc};

impl SkimItem for DirItem {
    fn text(&self) -> std::borrow::Cow<str> {
        Cow::from(self.display.clone())
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
    items.iter().for_each(|item| {
        let _ = tx_items.send(Arc::new(item.to_owned()));
    });
    drop(tx_items); // indicates that all items have been sent
    rx_items
}

pub fn find(items: &[DirItem]) -> Option<String> {
    let mut skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .preview(if Settings::get_readonly().preview {
            Some("")
        } else {
            None
        })
        .multi(false)
        .build()
        .unwrap();

    skim_options.cmd_collector = Rc::new(RefCell::new(SkimItemReader::new(
        SkimItemReaderOption::default().ansi(true).build(),
    )));

    let items = receiver(items);

    Skim::run_with(&skim_options, Some(items))
        .map(|out| {
            let selected = out.selected_items.first();
            let selected = match selected {
                Some(item) => {
                    let selected_dir = (**item).as_any().downcast_ref::<DirItem>().unwrap();
                    Some(selected_dir)
                }
                None => None,
            };

            if let Some(selected) = selected {
                if out.final_key == Key::Enter {
                    return Some(selected.path.to_str().unwrap().to_string());
                }
            }

            None
        })
        .flatten()
}
