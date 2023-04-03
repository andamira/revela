// revela::event::keys
//
//! Keyboard events.
//

mod code;
mod mods;

pub use code::{FunctionKey, KeyCode, MediaKey, ModifierKey};
pub use mods::KeyModifiers;

use super::{Event, EventKind};

/// Keyboard events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub mods: KeyModifiers,
    pub kind: KeyKind,
    // pub state: KeyEventState,
}

/// Keyboard event kind.
///
/// # For terminal emulators
/// - Only `Press` is supported by every terminal.
/// - Windows could additionally support `Release`.
/// - The kitty terminal also supports `Release` and `Repeat` on any OS.
//
// Support key release events for windows https://github.com/crossterm-rs/crossterm/pull/745
// kitty keyboard protocol: https://sw.kovidgoyal.net/kitty/keyboard-protocol/#modifiers
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyKind {
    /// The press of a key.
    #[default]
    Press,
    /// The release of a key.
    Release,
    /// The automatic repetition of a key that's being maintained unreleased.
    Repeat,
}

impl KeyEvent {
    /// Returns a new keyboard event.
    // TODO: IMPROVE const? or impl Into<KeyModifiers?>
    pub fn new(code: KeyCode, mods: KeyModifiers, kind: KeyKind) -> Self {
        Self { code, mods, kind }
    }

    //

    /// Returns `true` if the kind is `KeyKind::Press`.
    pub fn is_press(&self) -> bool {
        matches![self.kind, KeyKind::Press]
    }
    /// Returns `true` if the kind is `KeyKind::Release`.
    pub fn is_release(&self) -> bool {
        matches![self.kind, KeyKind::Release]
    }
    /// Returns `true` if the kind is `KeyKind::Repeat`.
    pub fn is_repeat(&self) -> bool {
        matches![self.kind, KeyKind::Repeat]
    }

    //

    /// Returns `true` if the code is `KeyCode::Char(char)`.
    #[inline]
    pub fn is_char(&self) -> bool {
        matches![self.code, KeyCode::Char(_)]
    }

    /// Returns `true` if the code is `KeyCode::F(FunctionKey)`.
    #[inline]
    pub fn is_function(&self) -> bool {
        matches![self.code, KeyCode::F(_)]
    }

    /// Returns `true` if the code is `KeyCode::Media(MediaKey)`.
    #[inline]
    pub fn is_media(&self) -> bool {
        matches![self.code, KeyCode::F(_)]
    }

    /// Returns `true` if the code is one of the arrow keys ←↑↓→ .
    #[inline]
    pub fn is_arrow(&self) -> bool {
        use KeyCode::*;
        matches![self.code, Up | Left | Right | Down]
    }

    //

    /// Returns some `char` if the event code is `KeyCode::Char(char)`.
    #[inline]
    #[rustfmt::skip]
    pub fn some_char(&self) -> Option<char> {
        if let KeyCode::Char(c) = &self.code { Some(*c) } else { None }
    }

    /// Returns some function key number if the event code is `KeyCode::F(FunctionKey)`.
    #[inline]
    #[rustfmt::skip]
    pub fn some_function(&self) -> Option<FunctionKey> {
        if let KeyCode::F(c) = &self.code { Some(*c) } else { None }
    }
}

// TEMP IMPROVE From<(KeyCode, KeyModifiers, KeyEventKind)> for KeyEvent {
impl From<(KeyCode, KeyModifiers, KeyKind)> for KeyEvent {
    fn from(t: (KeyCode, KeyModifiers, KeyKind)) -> KeyEvent {
        KeyEvent {
            code: t.0,
            mods: t.1,
            kind: t.2,
        }
    }
}
// TEMP IMPROVE
impl From<(MediaKey, KeyModifiers, KeyKind)> for KeyEvent {
    fn from(t: (MediaKey, KeyModifiers, KeyKind)) -> KeyEvent {
        KeyEvent {
            code: t.0.into(),
            mods: t.1,
            kind: t.2,
        }
    }
}
// TEMP IMPROVE
// impl From<ModifierKey> for KeyEvent {
impl From<(ModifierKey, KeyModifiers, KeyKind)> for KeyEvent {
    fn from(t: (ModifierKey, KeyModifiers, KeyKind)) -> KeyEvent {
        // RETHINK: duplication?? CHECK Kitty
        KeyEvent {
            code: t.0.into(),
            mods: t.1,
            kind: t.2,
        }
    }
}

/* for EventKind */

impl From<KeyEvent> for EventKind {
    #[inline]
    fn from(key_event: KeyEvent) -> EventKind {
        EventKind::Key(key_event)
    }
}

impl From<(KeyCode, KeyModifiers, KeyKind)> for EventKind {
    #[inline]
    fn from(t: (KeyCode, KeyModifiers, KeyKind)) -> EventKind {
        KeyEvent::from(t).into()
    }
}
impl From<(KeyCode, KeyModifiers)> for EventKind {
    #[inline]
    fn from(t: (KeyCode, KeyModifiers)) -> EventKind {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}

impl From<(MediaKey, KeyModifiers, KeyKind)> for EventKind {
    #[inline]
    fn from(t: (MediaKey, KeyModifiers, KeyKind)) -> EventKind {
        KeyEvent::from(t).into()
    }
}
impl From<(MediaKey, KeyModifiers)> for EventKind {
    #[inline]
    fn from(t: (MediaKey, KeyModifiers)) -> EventKind {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}

impl From<(ModifierKey, KeyModifiers, KeyKind)> for EventKind {
    #[inline]
    fn from(t: (ModifierKey, KeyModifiers, KeyKind)) -> EventKind {
        KeyEvent::from(t).into()
    }
}
impl From<(ModifierKey, KeyModifiers)> for EventKind {
    #[inline]
    fn from(t: (ModifierKey, KeyModifiers)) -> EventKind {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}

/* for Event */

impl From<KeyEvent> for Event {
    #[inline]
    fn from(key_event: KeyEvent) -> Event {
        EventKind::from(key_event).into()
    }
}
impl From<(KeyCode, KeyModifiers, KeyKind)> for Event {
    #[inline]
    fn from(t: (KeyCode, KeyModifiers, KeyKind)) -> Event {
        KeyEvent::from(t).into()
    }
}
impl From<(KeyCode, KeyModifiers)> for Event {
    #[inline]
    fn from(t: (KeyCode, KeyModifiers)) -> Event {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}

impl From<(MediaKey, KeyModifiers, KeyKind)> for Event {
    #[inline]
    fn from(t: (MediaKey, KeyModifiers, KeyKind)) -> Event {
        KeyEvent::from(t).into()
    }
}
impl From<(MediaKey, KeyModifiers)> for Event {
    #[inline]
    fn from(t: (MediaKey, KeyModifiers)) -> Event {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}

impl From<(ModifierKey, KeyModifiers, KeyKind)> for Event {
    #[inline]
    fn from(t: (ModifierKey, KeyModifiers, KeyKind)) -> Event {
        KeyEvent::from(t).into()
    }
}
impl From<(ModifierKey, KeyModifiers)> for Event {
    #[inline]
    fn from(t: (ModifierKey, KeyModifiers)) -> Event {
        KeyEvent::from((t.0, t.1, KeyKind::default())).into()
    }
}
