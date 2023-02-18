// revela::core::event::keys
//
//! Keyboard events.
//

mod code;
mod mods;

// MAYBE rename: all to KeyCode
pub use code::{KeyCode, MediaKey, ModifierKey};
pub use mods::KeyModifiers;

use super::{Event, EventKind};

///  Keyboard events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub mods: KeyModifiers,
    // pub kind: KeyEventKind,
    // pub state: KeyEventState,
}

impl KeyEvent {
    // TODO: IMPROVE const? or impl Into<KeyModifiers?>
    pub fn new(code: KeyCode, mods: KeyModifiers /*kind: KeyEventKind*/) -> Self {
        Self { code, mods }
    }
}

// TEMP IMPROVE From<(KeyCode, KeyModifiers, KeyEventKind)> for KeyEvent {
impl From<(KeyCode, KeyModifiers)> for KeyEvent {
    fn from(t: (KeyCode, KeyModifiers)) -> KeyEvent {
        KeyEvent {
            code: t.0,
            mods: t.1,
        }
    }
}
// TEMP IMPROVE
impl From<(MediaKey, KeyModifiers)> for KeyEvent {
    fn from(t: (MediaKey, KeyModifiers)) -> KeyEvent {
        KeyEvent {
            code: t.0.into(),
            mods: t.1,
        }
    }
}
// TEMP IMPROVE
// impl From<ModifierKey> for KeyEvent {
impl From<(ModifierKey, KeyModifiers)> for KeyEvent {
    fn from(t: (ModifierKey, KeyModifiers)) -> KeyEvent {
        // RETHINK: duplication?? CHECK Kitty
        KeyEvent {
            code: t.0.into(),
            mods: t.1,
        }
    }
}

impl From<KeyEvent> for EventKind {
    fn from(key_event: KeyEvent) -> EventKind {
        EventKind::Key(key_event)
    }
}
// MAYBE:
impl From<(KeyCode, KeyModifiers)> for EventKind {
    fn from(t: (KeyCode, KeyModifiers)) -> EventKind {
        KeyEvent::from(t).into()
    }
}
impl From<(MediaKey, KeyModifiers)> for EventKind {
    fn from(t: (MediaKey, KeyModifiers)) -> EventKind {
        log::debug!["nc: converting to event…"];
        KeyEvent::from(t).into()
    }
}
impl From<(ModifierKey, KeyModifiers)> for EventKind {
    // RETHINK duplication?
    fn from(t: (ModifierKey, KeyModifiers)) -> EventKind {
        KeyEvent::from(t).into()
    }
}

impl From<KeyEvent> for Event {
    fn from(key_event: KeyEvent) -> Event {
        EventKind::from(key_event).into()
    }
}
// MAYBE:
impl From<(KeyCode, KeyModifiers)> for Event {
    fn from(t: (KeyCode, KeyModifiers)) -> Event {
        KeyEvent::from(t).into()
    }
}
impl From<(MediaKey, KeyModifiers)> for Event {
    fn from(t: (MediaKey, KeyModifiers)) -> Event {
        log::debug!["nc: converting to event…"];
        KeyEvent::from(t).into()
    }
}
impl From<(ModifierKey, KeyModifiers)> for Event {
    // RETHINK duplication?
    fn from(t: (ModifierKey, KeyModifiers)) -> Event {
        KeyEvent::from(t).into()
    }
}
