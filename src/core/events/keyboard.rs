// revela::core::event::keyboard
//
//! Keyboard events.
//

#![allow(non_upper_case_globals)]

use super::{Event, EventKind};

///  Keyboard events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub mods: KeyModifiers,
    // WIP
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

/// Keyboard codes.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Code.html
// - https://docs.rs/notcurses/latest/notcurses/struct.Code.html
//   - https://github.com/dankamongmen/notcurses/blob/b0f19f9f296bed44ee2ca69b0cbff1b2b29902f0/src/lib/in.c#L180
// - https://man.archlinux.org/man/terminfo.5.en
// - https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Keycode.html (235)
//   - https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Scancode.html (241)
//   - NOTE difference with scan code https://stackoverflow.com/a/57124957/940200
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyCode {
    // mapped to:
    // - crossterm: Null,
    Unknown,

    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,

    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,

    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,

    /// Delete key.
    Delete,
    /// Insert key.
    Insert,

    /// Function keys.
    ///
    /// - Normal: F1-F12
    /// - +Shift: F13-F24
    /// - +Control: F25-F36
    /// - +Shift+Control: F37-F48
    F(u8),

    /// A unicode character.
    Char(char),

    /// Tab key.
    Tab,
    // BackTab, // crossterm returns this from Shift + Tab
    /// Escape key.
    Escape,

    // NOTE: from crossterm this is received as ' ' character
    Space,

    /// Caps Lock key.
    CapsLock,
    /// Scroll Lock key.
    ScrollLock,

    /// Num Lock key.
    NumLock,

    /// Print Screen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,

    // MAYBE
    // Keypad(KeypadKey)
    // KeypadBegin,
    Media(MediaKey),
    Modifier(ModifierKey),
}
/// # aliases
impl KeyCode {
    /// Alias of [`Escape`][KeyCode::Escape].
    pub const Esc: KeyCode = KeyCode::Escape;
    /// Alias of [`Insert`][KeyCode::Insert].
    pub const Ins: KeyCode = KeyCode::Insert;
    /// Alias of [`Delete`][KeyCode::Delete].
    pub const Del: KeyCode = KeyCode::Delete;
}

/// Media key codes.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.MediaKey.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MediaKey {
    Play,
    Pause,
    PlayPause,
    Reverse,
    Stop,
    FastForward,
    Rewind,
    Next,
    Previous,
    Record,
    LowerVolume,
    RaiseVolume,
    MuteVolume,
}
impl From<MediaKey> for KeyCode {
    fn from(code: MediaKey) -> KeyCode {
        KeyCode::Media(code)
    }
}

/// Modifier key codes (when pressed by themselves)
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.ModifierKey.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModifierKey {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}
impl From<ModifierKey> for KeyCode {
    fn from(code: ModifierKey) -> KeyCode {
        KeyCode::Modifier(code)
    }
}
impl ModifierKey {
    /// AltGr key.
    pub const AltGr: ModifierKey = ModifierKey::IsoLevel3Shift;
}

/* modifiers */

bitflags::bitflags! {
    #[derive(Default)]
    struct InnerKeyModifiers: u8 {
        const None = 0;
        const All = 0b1111_1111;

        // definitions come straight from the kitty keyboard protocol & notcurses.
        const Shift = 1;
        const Alt = 2;
        const Control = 4;
        const Super = 8;
        const Hyper = 16;
        const Meta = 32;
        const CapsLock = 64;
        const NumLock = 128;
    }
}
impl From<InnerKeyModifiers> for KeyModifiers {
    #[inline]
    fn from(inner: InnerKeyModifiers) -> KeyModifiers {
        KeyModifiers(inner)
    }
}
impl From<KeyModifiers> for InnerKeyModifiers {
    #[inline]
    fn from(outer: KeyModifiers) -> InnerKeyModifiers {
        outer.0
    }
}
impl From<u8> for InnerKeyModifiers {
    #[inline]
    fn from(int: u8) -> Self {
        InnerKeyModifiers::from_bits_truncate(int)
    }
}
impl From<u8> for KeyModifiers {
    #[inline]
    fn from(int: u8) -> Self {
        KeyModifiers(int.into())
    }
}

// MAYBE
// /// An enumeration of key modifiers.
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum KeyModifier {
//     Shift = InnerKeyModifiers::Shift.bits(),
//     Control = InnerKeyModifiers::Control.bits(),
//     Alt = InnerKeyModifiers::Alt.bits(),
//     Super = InnerKeyModifiers::Super.bits(),
//     Hyper = InnerKeyModifiers::Hyper.bits(),
//     Meta = InnerKeyModifiers::Meta.bits(),
//     CapsLock = InnerKeyModifiers::CapsLock.bits(),
//     NumLock = InnerKeyModifiers::NumLock.bits(),
// }
// impl From<KeyModifier> for InnerKeyModifiers {
//     #[inline]
//     fn from(km: KeyModifier) -> Self {
//         InnerKeyModifiers::from_bits_truncate(km as u8)
//     }
// }
// impl From<&KeyModifier> for InnerKeyModifiers {
//     #[inline]
//     fn from(km: &KeyModifier) -> Self {
//         InnerKeyModifiers::from_bits_truncate(*km as u8)
//     }
// }
// impl From<&[KeyModifier]> for InnerKeyModifiers {
//     fn from(slice: &[KeyModifier]) -> Self {
//         let mut ikms = InnerKeyModifiers::None;
//         for km in slice {
//             ikms.insert(km.into());
//         }
//         ikms
//     }
// }
// impl From<KeyModifier> for KeyModifiers {
//     #[inline]
//     fn from(km: KeyModifier) -> Self {
//         KeyModifiers(km.into())
//     }
// }
// impl From<&KeyModifier> for KeyModifiers {
//     #[inline]
//     fn from(km: &KeyModifier) -> Self {
//         KeyModifiers(km.into())
//     }
// }
// impl From<&[KeyModifier]> for KeyModifiers {
//     fn from(slice: &[KeyModifier]) -> Self {
//         let mut kms = KeyModifiers::None;
//         for km in slice {
//             kms.0.insert(km.into());
//         }
//         kms
//     }
// }

/// A bitmask of key modifiers.
//
// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html
// https://docs.rs/notcurses/latest/notcurses/struct.KeyMod.html
// https://docs.rs/sdl2/latest/sdl2/keyboard/struct.Mod.html
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyModifiers(InnerKeyModifiers);
#[rustfmt::skip]
impl KeyModifiers {
    pub const Shift: Self = Self(InnerKeyModifiers::Shift);
    pub const Control: Self = Self(InnerKeyModifiers::Control);
    pub const Alt: Self = Self(InnerKeyModifiers::Alt);
    pub const Super: Self = Self(InnerKeyModifiers::Super);
    pub const Hyper: Self = Self(InnerKeyModifiers::Hyper);
    pub const Meta: Self = Self(InnerKeyModifiers::Meta);
    pub const CapsLock: Self = Self(InnerKeyModifiers::CapsLock);
    pub const NumLock: Self = Self(InnerKeyModifiers::NumLock);
}
impl KeyModifiers {
    pub const None: Self = Self(InnerKeyModifiers::None);
    pub const All: Self = Self(InnerKeyModifiers::All);
}

impl KeyModifiers {
    // MAYBE
    // /// Returns `true` if the bitmask contains the provided `modifier`.
    // #[inline]
    // pub fn has(&self, modifier: impl Into<KeyModifier>) -> bool {
    //     self.0.intersects(modifier.into().into())
    // }
    // MAYBE
    // /// Returns `true` if the bitmask contains any of the provided `modifiers`.
    // #[inline]
    // pub fn has_any(&self, modifiers: &[impl Into<KeyModifier>]) -> bool {
    //     // let ikms:  = modifiers.iter().map(|m| m.into()).collect();
    //     // self.0.intersects(ikms)
    //     false
    // }

    /// Returns `true` if the bitmask contains no modifiers.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the bitmask contains any of the provided `modifiers`.
    #[inline]
    pub const fn has_any(&self, modifiers: KeyModifiers) -> bool {
        self.0.intersects(modifiers.0)
    }

    /// Returns `true` if the bitmask contains all of the provided `modifiers`.
    #[inline]
    pub const fn has_all(&self, modifiers: KeyModifiers) -> bool {
        self.0.contains(modifiers.0)
    }

    /// Inserts the specified `modifiers` in place.
    #[inline]
    pub fn insert(&mut self, modifiers: KeyModifiers) {
        self.0.insert(modifiers.0)
    }

    /// Removes the specified `modifiers` in place.
    #[inline]
    pub fn remove(&mut self, modifiers: KeyModifiers) {
        self.0.remove(modifiers.0)
    }

    /// Toggles the specified `modifiers` in place.
    #[inline]
    pub fn toggle(&mut self, modifiers: KeyModifiers) {
        self.0.toggle(modifiers.0)
    }

    // MAYBE
    // /// Inserts or removes the specified `modifiers`, depending on the `condition`.
    // #[inline]
    // pub fn set_if(&mut self, modifiers: KeyModifiers, condition: bool) {
    //     self.0.set(modifiers.0, condition)
    // }

    // /// Returns a complement bitmask with the missing modifiers.
    // #[inline]
    // pub const fn complement(&self) -> KeyModifiers {
    //     KeyModifiers(self.0.complement())
    // }

    /// Returns a complement bitmask with the missing modifiers.
    #[inline]
    pub const fn complement(&self) -> KeyModifiers {
        KeyModifiers(self.0.complement())
    }
}

impl KeyModifiers {
    /* get */

    /// Returns `true` if the bitmask contains the *Shift* modifier.
    #[inline]
    pub const fn shift(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Shift)
    }
    /// Returns `true` if the bitmask contains the *Control* modifier.
    #[inline]
    pub const fn control(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Control)
    }
    // /// Alias of [`control`][Self::control]
    // pub const fn ctrl(&self) -> bool { self.control() }
    /// Returns `true` if the bitmask contains the *Alt* modifier.
    #[inline]
    pub const fn alt(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Alt)
    }
    /// Returns `true` if the bitmask contains the *Super* modifier.
    // NOTE: super is a reserved keyword.
    #[inline]
    pub const fn sup(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Super)
    }
    /// Returns `true` if the bitmask contains the *Hyper* modifier.
    #[inline]
    pub const fn hyper(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Hyper)
    }
    /// Returns `true` if the bitmask contains the *Meta* modifier.
    #[inline]
    pub const fn meta(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::Meta)
    }
    /// Returns `true` if the bitmask contains the *CapsLock* modifier.
    #[inline]
    pub const fn caps_lock(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::CapsLock)
    }
    /// Returns `true` if the bitmask contains the *NumLock* modifier.
    #[inline]
    pub const fn num_lock(&self) -> bool {
        self.0.intersects(InnerKeyModifiers::NumLock)
    }

    /* set */

    /// Sets the *Shift* modifier.
    #[inline]
    pub fn set_shift(&mut self) {
        self.0.insert(InnerKeyModifiers::Shift)
    }
    /// Sets the *Control* modifier.
    #[inline]
    pub fn set_control(&mut self) {
        self.0.insert(InnerKeyModifiers::Control)
    }
    // /// Alias of [`control`][Self::control]
    // pub fn set_ctrl(&mut self) { self.control() }
    /// Sets the *Alt* modifier.
    #[inline]
    pub fn set_alt(&mut self) {
        self.0.insert(InnerKeyModifiers::Alt)
    }
    /// Sets the *Super* modifier.
    // NOTE: super is a reserved keyword.
    #[inline]
    pub fn set_sup(&mut self) {
        self.0.insert(InnerKeyModifiers::Super)
    }
    /// Sets the *Hyper* modifier.
    #[inline]
    pub fn set_hyper(&mut self) {
        self.0.insert(InnerKeyModifiers::Hyper)
    }
    /// Sets the *Meta* modifier.
    #[inline]
    pub fn set_meta(&mut self) {
        self.0.insert(InnerKeyModifiers::Meta)
    }
    /// Sets the *CapsLock* modifier.
    #[inline]
    pub fn set_caps_lock(&mut self) {
        self.0.insert(InnerKeyModifiers::CapsLock)
    }
    /// Sets the *NumLock* modifier.
    #[inline]
    pub fn set_num_lock(&mut self) {
        self.0.insert(InnerKeyModifiers::NumLock)
    }

    /* toggle */

    /// Toggles the *Shift* modifier.
    #[inline]
    pub fn toggle_shift(&mut self) {
        self.0.toggle(InnerKeyModifiers::Shift)
    }
    /// Toggles the *Control* modifier.
    #[inline]
    pub fn toggle_control(&mut self) {
        self.0.toggle(InnerKeyModifiers::Control)
    }
    // /// Alias of [`control`][Self::control]
    // pub fn toggle_ctrl(&mut self) { self.control() }
    /// Toggles the *Alt* modifier.
    #[inline]
    pub fn toggle_alt(&mut self) {
        self.0.toggle(InnerKeyModifiers::Alt)
    }
    /// Toggles the *Super* modifier.
    // NOTE: super is a reserved keyword.
    #[inline]
    pub fn toggle_sup(&mut self) {
        self.0.toggle(InnerKeyModifiers::Super)
    }
    /// Toggles the *Hyper* modifier.
    #[inline]
    pub fn toggle_hyper(&mut self) {
        self.0.toggle(InnerKeyModifiers::Hyper)
    }
    /// Toggles the *Meta* modifier.
    #[inline]
    pub fn toggle_meta(&mut self) {
        self.0.toggle(InnerKeyModifiers::Meta)
    }
    /// Toggles the *CapsLock* modifier.
    #[inline]
    pub fn toggle_caps_lock(&mut self) {
        self.0.toggle(InnerKeyModifiers::CapsLock)
    }
    /// Toggles the *NumLock* modifier.
    #[inline]
    pub fn toggle_num_lock(&mut self) {
        self.0.toggle(InnerKeyModifiers::NumLock)
    }
}
