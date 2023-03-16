// revela::backend::crossterm::events
//
//! Events types conversions.
//
// TODO
// - [ ] KeyEventState -> discard for now: (KEYPAD, CAPS_LOCK, NUM_LOCK)
// - MAYBE add conversions back to crossterm
//
// MAYBE FIX
// - missing key combos: https://github.com/crossterm-rs/crossterm/issues/685
// - ctrl + bckspace: https://github.com/crossterm-rs/crossterm/issues/723

use devela::iif;

use crate::all::{
    Event, EventKind, KeyCode, KeyEvent, KeyKind, KeyModifiers, MediaKey, ModifierKey, MouseButton,
    MouseEvent, MouseKind, WindowEvent,
};
use ::crossterm::event::{
    Event as CtEvent, KeyCode as CtKeyCode, KeyEvent as CtKeyEvent, KeyEventKind as CtKeyEventKind,
    KeyModifiers as CtKeyModifiers, MediaKeyCode as CtMediaKeyCode,
    ModifierKeyCode as CtModifierKeyCode, MouseButton as CtMouseButton, MouseEvent as CtMouseEvent,
    MouseEventKind as CtMouseEventKind,
};

impl From<CtEvent> for Event {
    fn from(ct: CtEvent) -> Event {
        EventKind::from(ct).into()
    }
}

// https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
impl From<CtEvent> for EventKind {
    fn from(ct: CtEvent) -> EventKind {
        use CtEvent::*;
        match ct {
            Key(k) => KeyEvent::from(k).into(),
            FocusGained => WindowEvent::FocusGained.into(),
            FocusLost => WindowEvent::FocusLost.into(),
            Resize(w, h) => WindowEvent::Resized(Some((w, h).into())).into(),
            #[cfg(feature = "alloc")]
            Paste(s) => WindowEvent::Paste(s).into(),
            #[cfg(not(feature = "alloc"))]
            Paste(_) => EventKind::None,
            Mouse(m) => MouseEvent::from(m).into(),
        }
    }
}

// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
impl From<CtKeyEvent> for KeyEvent {
    fn from(ct: CtKeyEvent) -> KeyEvent {
        let km = KeyModifiers::from(ct.modifiers);
        let kk = KeyKind::from(ct.kind);
        let _ks = ct.state; // THINK

        use CtKeyCode::*;

        match ct.code {
            // special transformations
            // THINK about: Null, KeyPadBegin
            BackTab => (KeyCode::Tab, km.with_shift(true), kk).into(),
            _ => (KeyCode::from(ct.code), km, kk).into(),
        }
    }
}

// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html
impl From<CtKeyModifiers> for KeyModifiers {
    fn from(ct: CtKeyModifiers) -> KeyModifiers {
        let mut km = KeyModifiers::None;

        if ct.is_empty() {
            km
        } else {
            iif![ct.intersects(CtKeyModifiers::SHIFT); km.set_shift()];
            iif![ct.intersects(CtKeyModifiers::CONTROL); km.set_control()];
            iif![ct.intersects(CtKeyModifiers::ALT); km.set_alt()];
            iif![ct.intersects(CtKeyModifiers::SUPER); km.set_sup()];
            iif![ct.intersects(CtKeyModifiers::HYPER); km.set_hyper()];
            iif![ct.intersects(CtKeyModifiers::META); km.set_meta()];
            // no caps_lock
            // no num_lock
            km
        }
    }
}

impl From<CtModifierKeyCode> for ModifierKey {
    fn from(ct: CtModifierKeyCode) -> ModifierKey {
        use CtModifierKeyCode::*;
        match ct {
            LeftShift => ModifierKey::LeftShift,
            LeftControl => ModifierKey::LeftControl,
            LeftAlt => ModifierKey::LeftAlt,
            LeftSuper => ModifierKey::LeftSuper,
            LeftHyper => ModifierKey::LeftHyper,
            LeftMeta => ModifierKey::LeftMeta,
            RightShift => ModifierKey::RightShift,
            RightControl => ModifierKey::RightControl,
            RightAlt => ModifierKey::RightAlt,
            RightSuper => ModifierKey::RightSuper,
            RightHyper => ModifierKey::RightHyper,
            RightMeta => ModifierKey::RightMeta,
            IsoLevel3Shift => ModifierKey::IsoLevel3Shift,
            IsoLevel5Shift => ModifierKey::IsoLevel5Shift,
        }
    }
}

impl From<CtMediaKeyCode> for MediaKey {
    fn from(ct: CtMediaKeyCode) -> MediaKey {
        use CtMediaKeyCode::*;
        match ct {
            Play => MediaKey::Play,
            Pause => MediaKey::Pause,
            PlayPause => MediaKey::PlayPause,
            Reverse => MediaKey::Reverse,
            Stop => MediaKey::Stop,
            FastForward => MediaKey::FastForward,
            Rewind => MediaKey::Rewind,
            TrackNext => MediaKey::Next,
            TrackPrevious => MediaKey::Previous,
            Record => MediaKey::Record,
            LowerVolume => MediaKey::LowerVolume,
            RaiseVolume => MediaKey::RaiseVolume,
            MuteVolume => MediaKey::MuteVolume,
        }
    }
}

impl From<CtKeyCode> for KeyCode {
    fn from(ct: CtKeyCode) -> KeyCode {
        use CtKeyCode::*;
        match ct {
            Backspace => KeyCode::Backspace,
            Enter => KeyCode::Enter,
            Left => KeyCode::Left,
            Right => KeyCode::Right,
            Up => KeyCode::Up,
            Down => KeyCode::Down,
            Home => KeyCode::Home,
            End => KeyCode::End,
            PageUp => KeyCode::PageUp,
            PageDown => KeyCode::PageDown,
            Tab => KeyCode::Tab,
            BackTab => KeyCode::Tab, // TODO take Shift into account in KeyEvent conversion
            Delete => KeyCode::Delete,
            Insert => KeyCode::Insert,
            F(f) => KeyCode::F(f),
            Char(c) => KeyCode::Char(c),
            Null => KeyCode::Unknown, // NOTE
            Esc => KeyCode::Escape,
            CapsLock => KeyCode::CapsLock,
            ScrollLock => KeyCode::ScrollLock,
            NumLock => KeyCode::NumLock,
            PrintScreen => KeyCode::PrintScreen,
            Pause => KeyCode::Pause,
            Menu => KeyCode::Menu,
            KeypadBegin => KeyCode::Unknown, // NOTE
            Media(m) => KeyCode::Media(m.into()),
            Modifier(m) => KeyCode::Modifier(m.into()),
        }
    }
}

impl From<CtKeyEventKind> for KeyKind {
    fn from(ct: CtKeyEventKind) -> KeyKind {
        use CtKeyEventKind::*;
        match ct {
            Press => KeyKind::Press,
            Repeat => KeyKind::Repeat,
            Release => KeyKind::Release,
        }
    }
}

/* mouse */

// https://docs.rs/crossterm/latest/crossterm/event/struct.MouseEvent.html
impl From<CtMouseEvent> for MouseEvent {
    fn from(ct: CtMouseEvent) -> MouseEvent {
        let button;
        let kind;
        {
            use CtMouseEventKind::*;
            match ct.kind {
                Down(b) => {
                    button = Some(b.into());
                    kind = MouseKind::Press;
                }
                Up(b) => {
                    button = Some(b.into());
                    kind = MouseKind::Release;
                }
                Drag(b) => {
                    button = Some(b.into());
                    // RETHINK
                    kind = MouseKind::Motion;
                }
                Moved => {
                    button = None;
                    kind = MouseKind::Motion;
                }
                ScrollDown => {
                    button = None;
                    kind = MouseKind::ScrollDown;
                }
                ScrollUp => {
                    button = None;
                    kind = MouseKind::ScrollUp;
                }
            }
        }

        MouseEvent {
            button,
            kind,
            // button: MouseButton::Left, // TEMP
            // kind: MouseKind::Press, // TEMP
            mods: ct.modifiers.into(),
            pos: (ct.column, ct.row).into(),
            offset: None,
        }
    }
}

// https://docs.rs/crossterm/latest/crossterm/event/enum.MouseButton.html
impl From<CtMouseButton> for MouseButton {
    fn from(ct: CtMouseButton) -> MouseButton {
        use CtMouseButton::*;
        match ct {
            Left => MouseButton::Left,
            Right => MouseButton::Right,
            Middle => MouseButton::Middle,
        }
    }
}
