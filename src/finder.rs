use crate::dirs::DirItem;
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
}

fn receiver(items: &[DirItem]) -> SkimItemReceiver {
    let (tx_items, rx_items): (SkimItemSender, SkimItemReceiver) = unbounded();
    items.iter().for_each(|feature| {
        let _ = tx_items.send(Arc::new(feature.to_owned()));
    });
    drop(tx_items); // indicates that all items have been sent
    rx_items
}

pub fn find(items: &[DirItem]) {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let receiver = receiver(items);

    let _ = Skim::run_with(&skim_options, Some(receiver)).map(|out| {
        if out.final_key != Key::Enter {
            return;
        }

        let selected = out.selected_items.first();
        match selected {
            Some(item) => {
                let selected_dir = (**item).as_any().downcast_ref::<DirItem>().unwrap();
                println!("{:?}", selected_dir);
            }
            None => {}
        }
    });
}
