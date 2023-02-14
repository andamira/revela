// revela::backend::nc::events
//
//! Events types conversions.
//
// TODO: KeyEvent

use crate::all::{Code, Event, KeyModifiers, MediaKey, ModifierKey, WindowEvent};
use ::notcurses::{Input, Key as NcKey, KeyMod, Received};

impl From<KeyMod> for KeyModifiers {
    fn from(nc: KeyMod) -> KeyModifiers {
        (u32::from(nc) as u8).into()
    }
}

impl From<Input> for Event {
    fn from(input: Input) -> Event {
        // 1. modifiers
        // - https://docs.rs/notcurses/latest/notcurses/struct.KeyMod.html

        // 2. type//kind

        match input.received {
            Received::Char(c) => (Code::Char(c), input.keymod.into()).into(),
            Received::Key(k) => {
                match k {
                    NcKey::Invalid => Event::None,

                    // we received `sigwinch`.
                    NcKey::Resize => WindowEvent::Resized.into(),
                    // we received sigcont
                    NcKey::Signal => WindowEvent::Continue.into(),
                    // will be returned upon reaching the end of input.
                    NcKey::Eof => WindowEvent::EndOfInput.into(),

                    NcKey::Up => (Code::Up, input.keymod.into()).into(),
                    NcKey::Right => (Code::Right, input.keymod.into()).into(),
                    NcKey::Down => (Code::Down, input.keymod.into()).into(),
                    NcKey::Left => (Code::Left, input.keymod.into()).into(),
                    NcKey::Ins => (Code::Insert, input.keymod.into()).into(),
                    NcKey::Del => (Code::Delete, input.keymod.into()).into(),
                    NcKey::Backspace => (Code::Backspace, input.keymod.into()).into(),
                    NcKey::PgDown => (Code::PageDown, input.keymod.into()).into(),
                    NcKey::PgUp => (Code::PageUp, input.keymod.into()).into(),
                    NcKey::Home => (Code::Home, input.keymod.into()).into(),
                    NcKey::End => (Code::End, input.keymod.into()).into(),

                    // CHECK:
                    // - Normal: F1-F12
                    // - +Shift: F13-F24
                    // - +Control: F25-F36
                    // - +Shift+Control: F37-F48
                    NcKey::F00 => (Code::F(0), input.keymod.into()).into(),
                    NcKey::F01 => (Code::F(1), input.keymod.into()).into(),
                    NcKey::F02 => (Code::F(2), input.keymod.into()).into(),
                    NcKey::F03 => (Code::F(3), input.keymod.into()).into(),
                    NcKey::F04 => (Code::F(4), input.keymod.into()).into(),
                    NcKey::F05 => (Code::F(5), input.keymod.into()).into(),
                    NcKey::F06 => (Code::F(6), input.keymod.into()).into(),
                    NcKey::F07 => (Code::F(7), input.keymod.into()).into(),
                    NcKey::F08 => (Code::F(8), input.keymod.into()).into(),
                    NcKey::F09 => (Code::F(9), input.keymod.into()).into(),
                    NcKey::F10 => (Code::F(10), input.keymod.into()).into(),
                    NcKey::F11 => (Code::F(11), input.keymod.into()).into(),
                    NcKey::F12 => (Code::F(12), input.keymod.into()).into(),
                    NcKey::F13 => (Code::F(13), input.keymod.into()).into(),
                    NcKey::F14 => (Code::F(14), input.keymod.into()).into(),
                    NcKey::F15 => (Code::F(15), input.keymod.into()).into(),
                    NcKey::F16 => (Code::F(16), input.keymod.into()).into(),
                    NcKey::F17 => (Code::F(17), input.keymod.into()).into(),
                    NcKey::F18 => (Code::F(18), input.keymod.into()).into(),
                    NcKey::F19 => (Code::F(19), input.keymod.into()).into(),
                    NcKey::F20 => (Code::F(20), input.keymod.into()).into(),
                    NcKey::F21 => (Code::F(21), input.keymod.into()).into(),
                    NcKey::F22 => (Code::F(22), input.keymod.into()).into(),
                    NcKey::F23 => (Code::F(23), input.keymod.into()).into(),
                    NcKey::F24 => (Code::F(24), input.keymod.into()).into(),
                    NcKey::F25 => (Code::F(25), input.keymod.into()).into(),
                    NcKey::F26 => (Code::F(26), input.keymod.into()).into(),
                    NcKey::F27 => (Code::F(27), input.keymod.into()).into(),
                    NcKey::F28 => (Code::F(28), input.keymod.into()).into(),
                    NcKey::F29 => (Code::F(29), input.keymod.into()).into(),
                    NcKey::F30 => (Code::F(30), input.keymod.into()).into(),
                    NcKey::F31 => (Code::F(31), input.keymod.into()).into(),
                    NcKey::F32 => (Code::F(32), input.keymod.into()).into(),
                    NcKey::F33 => (Code::F(33), input.keymod.into()).into(),
                    NcKey::F34 => (Code::F(34), input.keymod.into()).into(),
                    NcKey::F35 => (Code::F(35), input.keymod.into()).into(),
                    NcKey::F36 => (Code::F(36), input.keymod.into()).into(),
                    NcKey::F37 => (Code::F(37), input.keymod.into()).into(),
                    NcKey::F38 => (Code::F(38), input.keymod.into()).into(),
                    NcKey::F39 => (Code::F(39), input.keymod.into()).into(),
                    NcKey::F40 => (Code::F(40), input.keymod.into()).into(),
                    NcKey::F41 => (Code::F(41), input.keymod.into()).into(),
                    NcKey::F42 => (Code::F(42), input.keymod.into()).into(),
                    NcKey::F43 => (Code::F(43), input.keymod.into()).into(),
                    NcKey::F44 => (Code::F(44), input.keymod.into()).into(),
                    NcKey::F45 => (Code::F(45), input.keymod.into()).into(),
                    NcKey::F46 => (Code::F(46), input.keymod.into()).into(),
                    NcKey::F47 => (Code::F(47), input.keymod.into()).into(),
                    NcKey::F48 => (Code::F(48), input.keymod.into()).into(),
                    NcKey::F49 => (Code::F(49), input.keymod.into()).into(),
                    NcKey::F50 => (Code::F(50), input.keymod.into()).into(),
                    NcKey::F51 => (Code::F(51), input.keymod.into()).into(),
                    NcKey::F52 => (Code::F(52), input.keymod.into()).into(),
                    NcKey::F53 => (Code::F(53), input.keymod.into()).into(),
                    NcKey::F54 => (Code::F(54), input.keymod.into()).into(),
                    NcKey::F55 => (Code::F(55), input.keymod.into()).into(),
                    NcKey::F56 => (Code::F(56), input.keymod.into()).into(),
                    NcKey::F57 => (Code::F(57), input.keymod.into()).into(),
                    NcKey::F58 => (Code::F(58), input.keymod.into()).into(),
                    NcKey::F59 => (Code::F(59), input.keymod.into()).into(),
                    NcKey::F60 => (Code::F(60), input.keymod.into()).into(),
                    // ... leave room for function keys.
                    NcKey::Enter => (Code::Enter, input.keymod.into()).into(),

                    /*
                    // https://en.wikipedia.org/wiki/Numeric_keypad

                    // "clear-screen or erase"
                    NcKey::Cls => Code::None,
                    // down + left on keypad
                    NcKey::DLeft => Code::None,
                    NcKey::DRight => Code::None,
                    // up + left on keypad
                    NcKey::ULeft => Code::None,
                    NcKey::URight => Code::None,
                    NcKey::Center => Code::None,
                    NcKey::Begin => Code::None,
                    NcKey::Cancel => Code::None,
                    NcKey::Close => Code::None,
                    NcKey::Command => Code::None,
                    NcKey::Copy => Code::None,
                    NcKey::Exit => Code::None,
                    NcKey::Print => Code::None,
                    NcKey::Refresh => Code::None,
                    */
                    // these keys aren't generally available outside of the kitty protocol
                    NcKey::CapsLock => (Code::CapsLock, input.keymod.into()).into(),
                    NcKey::ScrollLock => (Code::ScrollLock, input.keymod.into()).into(),
                    NcKey::NumLock => (Code::NumLock, input.keymod.into()).into(),
                    NcKey::PrintScreen => (Code::PrintScreen, input.keymod.into()).into(),
                    NcKey::Pause => (Code::Pause, input.keymod.into()).into(),
                    NcKey::Menu => (Code::Menu, input.keymod.into()).into(),

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
                    // but position information is embedded in the larger ncinput event => Code::None,
                    /*
                    NcKey::Motion => Code::None,

                    NcKey::Button1 => Code::None,
                    NcKey::Button2 => Code::None,
                    NcKey::Button3 => Code::None,
                    // scrollwheel up
                    NcKey::Button4 => Code::None,
                    // scrollwheel down
                    NcKey::Button5 => Code::None,
                    NcKey::Button6 => Code::None,
                    NcKey::Button7 => Code::None,
                    NcKey::Button8 => Code::None,
                    NcKey::Button9 => Code::None,
                    NcKey::Button10 => Code::None,
                    NcKey::Button11 => Code::None,
                    */
                    // aliases from the 128 characters common to ascii+utf8 => Code::None,
                    NcKey::Tab => (Code::Tab, input.keymod.into()).into(),
                    NcKey::Esc => (Code::Esc, input.keymod.into()).into(),
                    NcKey::Space => (Code::Space, input.keymod.into()).into(),

                    /* // WIP
                     */ //WIP
                    _ => Event::None,
                }
            }
            _ => Event::None,
        }
    }
}
