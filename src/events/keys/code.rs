// revela::core::event::keys::code
//
//! Keyboard event codes.
//

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
