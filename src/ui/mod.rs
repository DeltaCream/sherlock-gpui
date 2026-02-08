pub mod main_window;
pub mod search_bar;

use gpui::KeyBinding;
use serde::{Deserialize, Serialize};

use crate::ui::main_window::{Execute, FocusNext, FocusPrev, NextVar, OpenContext, PrevVar, Quit};

#[derive(Deserialize, Serialize, Hash, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UIFunction {
    Exit,

    ItemDown,
    ItemUp,
    ItemLeft,
    ItemRight,

    ArgNext,
    ArgPrev,

    Exec,
    ExecInplace,

    MultiSelect,

    ToggleContext,
    CloseContext,

    ClearBar,
    Backspace,

    ErrorPage,

    Shortcut,
}
impl UIFunction {
    pub fn into_bind(&self, key: &str) -> Option<KeyBinding> {
        match self {
            Self::Exit => Some(KeyBinding::new(key, Quit, None)),
            Self::ItemDown => Some(KeyBinding::new(key, FocusNext, None)),
            Self::ItemUp => Some(KeyBinding::new(key, FocusPrev, None)),
            Self::Exec => Some(KeyBinding::new(key, Execute, None)),
            Self::ArgNext => Some(KeyBinding::new(key, NextVar, None)),
            Self::ArgPrev => Some(KeyBinding::new(key, PrevVar, None)),
            Self::ToggleContext => Some(KeyBinding::new(key, OpenContext, None)),
            _ => None,
        }
    }
}
