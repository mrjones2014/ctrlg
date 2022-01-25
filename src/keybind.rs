use skim::prelude::Key;

pub trait CtrlgKeybind {
    /// Get the key code for the key, e.g. Key::Ctrlg('o') -> 'ctrl-o'
    fn key_code(&self) -> &str;
    /// Get the Skim action name to bind to, see `man sk` to see all actions
    fn action(&self) -> &str;
    /// Get the key code and action mapping string for the keybind,
    /// e.g. Key::AltEnter -> 'alt-enter:accept'
    fn binding_string(&self) -> String;
    /// Get the output prefix based on the key pressed
    /// e.g. Key::AltEnter => "ctrlg_edit:". Only returns
    /// `Some` for keys that are bound to the `accept` Skim action.
    fn result_prefix(&self) -> Option<&str>;
    /// Get the human-readable description of what the keybind does.
    fn description(&self) -> &str;
}

impl CtrlgKeybind for Key {
    fn key_code(&self) -> &str {
        match self {
            Key::Enter => "enter",
            Key::AltEnter => "alt-enter",
            Key::Alt('o') => "alt-o",
            Key::Ctrl('o') => "ctrl-o",
            Key::Ctrl('d') => "ctrl-d",
            Key::Ctrl('f') => "ctrl-f",
            Key::Tab => "tab",
            _ => unimplemented!("Unused keybind matched"),
        }
    }

    fn action(&self) -> &str {
        match self {
            Key::Enter => "accept",
            Key::AltEnter => "accept",
            Key::Alt('o') => "accept",
            Key::Ctrl('o') => "accept",
            Key::Ctrl('d') => "preview-up",
            Key::Ctrl('f') => "preview-down",
            Key::Tab => "accept",
            _ => unimplemented!("Unused keybind matched"),
        }
    }

    fn binding_string(&self) -> String {
        format!("{}:{}", self.key_code(), self.action())
    }

    fn result_prefix(&self) -> Option<&str> {
        match self {
            Key::Enter => Some(""),
            Key::AltEnter => Some("ctrlg_edit:"),
            Key::Alt('o') => Some("ctrlg_pushd:"),
            Key::Ctrl('o') => Some("ctrlg_notmux:"),
            Key::Tab => Some("ctrlg_insert:"),
            _ => None,
        }
    }

    fn description(&self) -> &str {
        match self {
            Key::Enter => "'cd' to the selected directory. Sends command to all tmux panes if $CTRLG_TMUX is 'true'.",
            Key::AltEnter => "'cd' to the selected directory (in all tmux panes if $CTRLG_TMUX is 'true'), then open $EDITOR (only in current tmux pane).",
            Key::Alt('o') => "Open $EDITOR to the specified directory without changing the shell working directory.",
            Key::Ctrl('o') => "'cd' to the selected directory in the current tmux pane only.",
            Key::Ctrl('d') => "Scroll preview up.",
            Key::Ctrl('f') => "Scroll preview down.",
            Key::Tab => "accept",
            _ => unimplemented!("Unused keybind matched"),
        }
    }
}

pub fn get_bound_keys() -> [Key; 7] {
    [
        Key::Enter,
        Key::AltEnter,
        Key::Alt('o'),
        Key::Ctrl('o'),
        Key::Ctrl('d'),
        Key::Ctrl('f'),
        Key::Tab,
    ]
}
