use crate::command_strs;
use crate::dir_item::DirItem;
use crate::keybind::{get_bound_keys, CtrlgKeybind};
use crate::settings::Settings;
use skim::prelude::*;
use skim::{prelude::unbounded, SkimItem, SkimItemReceiver, SkimItemSender};
use std::{borrow::Cow, sync::Arc};

impl SkimItem for DirItem {
    fn text(&self) -> std::borrow::Cow<str> {
        Cow::from(self.match_str.clone())
    }

    fn display<'a>(&'a self, _: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::parse(self.display.as_str())
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let settings = Settings::global();
        if self.readme.is_none() {
            if settings.preview_fallback_exa {
                return ItemPreview::Command(format!(
                    "{} \"{}\"",
                    command_strs::EXA.join(" "),
                    self.path.to_str().unwrap()
                ));
            }

            return ItemPreview::Command(format!(
                "{} \"{}\"",
                command_strs::LS.join(" "),
                self.path.to_str().unwrap()
            ));
        }

        let readme_path = self.readme.as_ref().unwrap();
        if settings.preview_with_glow {
            let mut glow_args = command_strs::GLOW.to_vec();
            let wrap_width_str = settings.glow_wrap_width.to_string();
            glow_args.push(&wrap_width_str);
            ItemPreview::Command(format!(
                "{} \"{}\"",
                glow_args.join(" "),
                readme_path.to_str().unwrap()
            ))
        } else if settings.preview_with_bat {
            let mut bat_args = command_strs::BAT.to_vec();
            let bat_theme_arg = format!("--theme={}", settings.colors.bat_theme);
            bat_args.push(bat_theme_arg.as_str());
            ItemPreview::Command(format!(
                "{} \"{}\"",
                bat_args.join(" "),
                readme_path.to_str().unwrap()
            ))
        } else {
            ItemPreview::Command(format!(
                "{} \"{}\"",
                command_strs::CAT.join(" "),
                readme_path.to_str().unwrap()
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
    let keybinds = get_bound_keys();
    let keybind_strs = keybinds
        .iter()
        .map(|key| key.binding_string())
        .collect::<Vec<String>>();
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .preview(if Settings::global().preview {
            Some("")
        } else {
            None
        })
        .bind(
            keybind_strs
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>(),
        )
        .multi(false)
        .build()
        .unwrap();

    let items = receiver(items);

    Skim::run_with(&skim_options, Some(items)).and_then(|out| {
        let selected = out.selected_items.first();
        let selected = match selected {
            Some(item) => {
                let selected_dir = (**item).as_any().downcast_ref::<DirItem>().unwrap();
                Some(selected_dir)
            }
            None => None,
        };

        if let Some(selected) = selected {
            let final_key = out.final_key;
            let path = selected.path.to_str().unwrap().to_string();
            final_key.handle(path.clone());
            return final_key
                .result_prefix()
                .map(|prefix| format!("{}{}", prefix, path));
        }

        None
    })
}
