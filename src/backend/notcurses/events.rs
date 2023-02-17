// revela::backend::nc::events
//
//! Events types conversions.
//
// TODO: KeyEvent

use crate::all::{Event, EventKind, KeyCode, KeyModifiers, MediaKey, ModifierKey, WindowEvent};
use ::notcurses::{Input, Key as NcKey, KeyMod, Received};

impl From<KeyMod> for KeyModifiers {
    fn from(nc: KeyMod) -> KeyModifiers {
        (u32::from(nc) as u8).into()
    }
}

impl From<Input> for Event {
    fn from(input: Input) -> Event {
        EventKind::from(input).into()
    }
}

impl From<Input> for EventKind {
    fn from(input: Input) -> EventKind {
        // 1. modifiers
        // - https://docs.rs/notcurses/latest/notcurses/struct.KeyMod.html

        // 2. type//kind

        match input.received {
            Received::Char(c) => (KeyCode::Char(c), input.keymod.into()).into(),
            Received::Key(k) => {
                match k {
                    NcKey::Invalid => EventKind::None,

                    // we received `sigwinch`.
                    NcKey::Resize => WindowEvent::Resized.into(),
                    // we received sigcont
                    NcKey::Signal => WindowEvent::Continue.into(),
                    // will be returned upon reaching the end of input.
                    NcKey::Eof => WindowEvent::EndOfInput.into(),

                    NcKey::Up => (KeyCode::Up, input.keymod.into()).into(),
                    NcKey::Right => (KeyCode::Right, input.keymod.into()).into(),
                    NcKey::Down => (KeyCode::Down, input.keymod.into()).into(),
                    NcKey::Left => (KeyCode::Left, input.keymod.into()).into(),
                    NcKey::Ins => (KeyCode::Insert, input.keymod.into()).into(),
                    NcKey::Del => (KeyCode::Delete, input.keymod.into()).into(),
                    NcKey::Backspace => (KeyCode::Backspace, input.keymod.into()).into(),
                    NcKey::PgDown => (KeyCode::PageDown, input.keymod.into()).into(),
                    NcKey::PgUp => (KeyCode::PageUp, input.keymod.into()).into(),
                    NcKey::Home => (KeyCode::Home, input.keymod.into()).into(),
                    NcKey::End => (KeyCode::End, input.keymod.into()).into(),

                    // CHECK:
                    // - Normal: F1-F12
                    // - +Shift: F13-F24
                    // - +Control: F25-F36
                    // - +Shift+Control: F37-F48
                    NcKey::F00 => (KeyCode::F(0), input.keymod.into()).into(),
                    NcKey::F01 => (KeyCode::F(1), input.keymod.into()).into(),
                    NcKey::F02 => (KeyCode::F(2), input.keymod.into()).into(),
                    NcKey::F03 => (KeyCode::F(3), input.keymod.into()).into(),
                    NcKey::F04 => (KeyCode::F(4), input.keymod.into()).into(),
                    NcKey::F05 => (KeyCode::F(5), input.keymod.into()).into(),
                    NcKey::F06 => (KeyCode::F(6), input.keymod.into()).into(),
                    NcKey::F07 => (KeyCode::F(7), input.keymod.into()).into(),
                    NcKey::F08 => (KeyCode::F(8), input.keymod.into()).into(),
                    NcKey::F09 => (KeyCode::F(9), input.keymod.into()).into(),
                    NcKey::F10 => (KeyCode::F(10), input.keymod.into()).into(),
                    NcKey::F11 => (KeyCode::F(11), input.keymod.into()).into(),
                    NcKey::F12 => (KeyCode::F(12), input.keymod.into()).into(),
                    NcKey::F13 => (KeyCode::F(13), input.keymod.into()).into(),
                    NcKey::F14 => (KeyCode::F(14), input.keymod.into()).into(),
                    NcKey::F15 => (KeyCode::F(15), input.keymod.into()).into(),
                    NcKey::F16 => (KeyCode::F(16), input.keymod.into()).into(),
                    NcKey::F17 => (KeyCode::F(17), input.keymod.into()).into(),
                    NcKey::F18 => (KeyCode::F(18), input.keymod.into()).into(),
                    NcKey::F19 => (KeyCode::F(19), input.keymod.into()).into(),
                    NcKey::F20 => (KeyCode::F(20), input.keymod.into()).into(),
                    NcKey::F21 => (KeyCode::F(21), input.keymod.into()).into(),
                    NcKey::F22 => (KeyCode::F(22), input.keymod.into()).into(),
                    NcKey::F23 => (KeyCode::F(23), input.keymod.into()).into(),
                    NcKey::F24 => (KeyCode::F(24), input.keymod.into()).into(),
                    NcKey::F25 => (KeyCode::F(25), input.keymod.into()).into(),
                    NcKey::F26 => (KeyCode::F(26), input.keymod.into()).into(),
                    NcKey::F27 => (KeyCode::F(27), input.keymod.into()).into(),
                    NcKey::F28 => (KeyCode::F(28), input.keymod.into()).into(),
                    NcKey::F29 => (KeyCode::F(29), input.keymod.into()).into(),
                    NcKey::F30 => (KeyCode::F(30), input.keymod.into()).into(),
                    NcKey::F31 => (KeyCode::F(31), input.keymod.into()).into(),
                    NcKey::F32 => (KeyCode::F(32), input.keymod.into()).into(),
                    NcKey::F33 => (KeyCode::F(33), input.keymod.into()).into(),
                    NcKey::F34 => (KeyCode::F(34), input.keymod.into()).into(),
                    NcKey::F35 => (KeyCode::F(35), input.keymod.into()).into(),
                    NcKey::F36 => (KeyCode::F(36), input.keymod.into()).into(),
                    NcKey::F37 => (KeyCode::F(37), input.keymod.into()).into(),
                    NcKey::F38 => (KeyCode::F(38), input.keymod.into()).into(),
                    NcKey::F39 => (KeyCode::F(39), input.keymod.into()).into(),
                    NcKey::F40 => (KeyCode::F(40), input.keymod.into()).into(),
                    NcKey::F41 => (KeyCode::F(41), input.keymod.into()).into(),
                    NcKey::F42 => (KeyCode::F(42), input.keymod.into()).into(),
                    NcKey::F43 => (KeyCode::F(43), input.keymod.into()).into(),
                    NcKey::F44 => (KeyCode::F(44), input.keymod.into()).into(),
                    NcKey::F45 => (KeyCode::F(45), input.keymod.into()).into(),
                    NcKey::F46 => (KeyCode::F(46), input.keymod.into()).into(),
                    NcKey::F47 => (KeyCode::F(47), input.keymod.into()).into(),
                    NcKey::F48 => (KeyCode::F(48), input.keymod.into()).into(),
                    NcKey::F49 => (KeyCode::F(49), input.keymod.into()).into(),
                    NcKey::F50 => (KeyCode::F(50), input.keymod.into()).into(),
                    NcKey::F51 => (KeyCode::F(51), input.keymod.into()).into(),
                    NcKey::F52 => (KeyCode::F(52), input.keymod.into()).into(),
                    NcKey::F53 => (KeyCode::F(53), input.keymod.into()).into(),
                    NcKey::F54 => (KeyCode::F(54), input.keymod.into()).into(),
                    NcKey::F55 => (KeyCode::F(55), input.keymod.into()).into(),
                    NcKey::F56 => (KeyCode::F(56), input.keymod.into()).into(),
                    NcKey::F57 => (KeyCode::F(57), input.keymod.into()).into(),
                    NcKey::F58 => (KeyCode::F(58), input.keymod.into()).into(),
                    NcKey::F59 => (KeyCode::F(59), input.keymod.into()).into(),
                    NcKey::F60 => (KeyCode::F(60), input.keymod.into()).into(),
                    // ... leave room for function keys.
                    NcKey::Enter => (KeyCode::Enter, input.keymod.into()).into(),

                    /*
                    // https://en.wikipedia.org/wiki/Numeric_keypad

                    // "clear-screen or erase"
                    NcKey::Cls => KeyCode::None,
                    // down + left on keypad
                    NcKey::DLeft => KeyCode::None,
                    NcKey::DRight => KeyCode::None,
                    // up + left on keypad
                    NcKey::ULeft => KeyCode::None,
                    NcKey::URight => KeyCode::None,
                    NcKey::Center => KeyCode::None,
                    NcKey::Begin => KeyCode::None,
                    NcKey::Cancel => KeyCode::None,
                    NcKey::Close => KeyCode::None,
                    NcKey::Command => KeyCode::None,
                    NcKey::Copy => KeyCode::None,
                    NcKey::Exit => KeyCode::None,
                    NcKey::Print => KeyCode::None,
                    NcKey::Refresh => KeyCode::None,
                    */
                    // these keys aren't generally available outside of the kitty protocol
                    NcKey::CapsLock => (KeyCode::CapsLock, input.keymod.into()).into(),
                    NcKey::ScrollLock => (KeyCode::ScrollLock, input.keymod.into()).into(),
                    NcKey::NumLock => (KeyCode::NumLock, input.keymod.into()).into(),
                    NcKey::PrintScreen => (KeyCode::PrintScreen, input.keymod.into()).into(),
                    NcKey::Pause => (KeyCode::Pause, input.keymod.into()).into(),
                    NcKey::Menu => (KeyCode::Menu, input.keymod.into()).into(),

                    // media keys, similarly only available through kitty's protocol
                    NcKey::MediaPlay => (MediaKey::Play, input.keymod.into()).into(),
                    NcKey::MediaPause => (MediaKey::Pause, input.keymod.into()).into(),
                    NcKey::MediaPPause => (MediaKey::PlayPause, input.keymod.into()).into(),
                    NcKey::MediaRev => (MediaKey::Reverse, input.keymod.into()).into(),
                    NcKey::MediaStop => (MediaKey::Stop, input.keymod.into()).into(),
                    NcKey::MediaFF => (MediaKey::FastForward, input.keymod.into()).into(),
                    NcKey::MediaRewind => (MediaKey::Rewind, input.keymod.into()).into(),
                    NcKey::MediaNext => (MediaKey::Next, input.keymod.into()).into(),
                    NcKey::MediaPrev => (MediaKey::Previous, input.keymod.into()).into(),
                    NcKey::MediaRecord => (MediaKey::Record, input.keymod.into()).into(),
                    NcKey::MediaLVol => (MediaKey::LowerVolume, input.keymod.into()).into(),
                    NcKey::MediaRVol => (MediaKey::RaiseVolume, input.keymod.into()).into(),
                    NcKey::MediaMute => (MediaKey::MuteVolume, input.keymod.into()).into(),

                    // modifiers when pressed by themselves. this ordering comes from the kitty
                    // keyboard protocol, and mustn't be changed without updating handlers
                    NcKey::LShift => (ModifierKey::LeftShift, input.keymod.into()).into(),
                    NcKey::LCtrl => (ModifierKey::LeftControl, input.keymod.into()).into(),
                    NcKey::LAlt => (ModifierKey::LeftAlt, input.keymod.into()).into(),
                    NcKey::LSuper => (ModifierKey::LeftSuper, input.keymod.into()).into(),
                    NcKey::LHyper => (ModifierKey::LeftHyper, input.keymod.into()).into(),
                    NcKey::LMeta => (ModifierKey::LeftMeta, input.keymod.into()).into(),
                    NcKey::RShift => (ModifierKey::RightShift, input.keymod.into()).into(),
                    NcKey::RCtrl => (ModifierKey::RightControl, input.keymod.into()).into(),
                    NcKey::RAlt => (ModifierKey::RightAlt, input.keymod.into()).into(),
                    NcKey::RSuper => (ModifierKey::RightSuper, input.keymod.into()).into(),
                    NcKey::RHyper => (ModifierKey::RightHyper, input.keymod.into()).into(),
                    NcKey::RMeta => (ModifierKey::RightMeta, input.keymod.into()).into(),
                    // `altgr` in european keyboards
                    NcKey::L3Shift => (ModifierKey::IsoLevel3Shift, input.keymod.into()).into(),
                    NcKey::L5Shift => (ModifierKey::IsoLevel5Shift, input.keymod.into()).into(),

                    // TODO WIP
                    // mouse events. we encode which button was pressed into the number,
                    // but position information is embedded in the larger ncinput event => KeyCode::None,
                    /*
                    NcKey::Motion => KeyCode::None,

                    NcKey::Button1 => KeyCode::None,
                    NcKey::Button2 => KeyCode::None,
                    NcKey::Button3 => KeyCode::None,
                    // scrollwheel up
                    NcKey::Button4 => KeyCode::None,
                    // scrollwheel down
                    NcKey::Button5 => KeyCode::None,
                    NcKey::Button6 => KeyCode::None,
                    NcKey::Button7 => KeyCode::None,
                    NcKey::Button8 => KeyCode::None,
                    NcKey::Button9 => KeyCode::None,
                    NcKey::Button10 => KeyCode::None,
                    NcKey::Button11 => KeyCode::None,
                    */
                    // aliases from the 128 characters common to ascii+utf8 => KeyCode::None,
                    NcKey::Tab => (KeyCode::Tab, input.keymod.into()).into(),
                    NcKey::Esc => (KeyCode::Esc, input.keymod.into()).into(),
                    NcKey::Space => (KeyCode::Space, input.keymod.into()).into(),

                    /* // WIP
                     */ //WIP
                    _ => EventKind::None,
                }
            }
            _ => EventKind::None,
        }
    }
}
