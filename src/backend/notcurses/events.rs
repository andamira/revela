// revela::backend::notcurses::events
//
//! Event types conversions.
//

use crate::all::{
    Event, EventKind, KeyCode, KeyKind, KeyModifiers, MediaKey, ModifierKey, MouseEvent, MouseKind,
    Position, WindowEvent,
};
use ::notcurses::{Input, InputType, Key as NcKey, KeyMod, Received};

// https://docs.rs/notcurses/3.2.2/notcurses/struct.KeyMod.html
impl From<KeyMod> for KeyModifiers {
    fn from(nc: KeyMod) -> KeyModifiers {
        (u32::from(nc) as u8).into()
    }
}

// https://docs.rs/notcurses/3.2.2/notcurses/struct.Input.html
// pub struct Input {
//     pub received: Received,
//     pub keymod: KeyMod,
//     pub itype: InputType,
//     pub cell: Option<Position>,
//     pub offset: Option<Position>,
// }
impl From<Input> for Event {
    fn from(input: Input) -> Event {
        EventKind::from(input).into()
    }
}

// https://docs.rs/notcurses/3.2.2/notcurses/enum.InputType.html
impl From<InputType> for KeyKind {
    fn from(input: InputType) -> KeyKind {
        use InputType::*;
        match input {
            Unknown | Press => KeyKind::Press,
            Release => KeyKind::Release,
            Repeat => KeyKind::Repeat,
        }
    }
}
impl From<InputType> for MouseKind {
    fn from(input: InputType) -> MouseKind {
        use InputType::*;
        match input {
            Unknown | Press => MouseKind::Press,
            Release => MouseKind::Release,
            Repeat => MouseKind::Press,
        }
    }
}

impl From<Input> for EventKind {
    fn from(input: Input) -> EventKind {
        let km = KeyModifiers::from(input.keymod);
        let kk = KeyKind::from(input.itype);
        let pos: Option<Position> = input.cell.map(|p| p.into());
        let off: Option<Position> = input.offset.map(|p| p.into());

        match input.received {
            Received::Char(c) => (KeyCode::Char(c), km).into(),
            Received::Key(key) => {
                match key {
                    NcKey::Invalid => EventKind::None,

                    // we received `sigwinch`.
                    NcKey::Resize => WindowEvent::Resized(None).into(),
                    // we received sigcont
                    NcKey::Signal => WindowEvent::Continue.into(),
                    // will be returned upon reaching the end of input.
                    NcKey::Eof => WindowEvent::EndOfInput.into(),

                    NcKey::Up => (KeyCode::Up, km, kk).into(),
                    NcKey::Right => (KeyCode::Right, km, kk).into(),
                    NcKey::Down => (KeyCode::Down, km, kk).into(),
                    NcKey::Left => (KeyCode::Left, km, kk).into(),
                    NcKey::Ins => (KeyCode::Insert, km, kk).into(),
                    NcKey::Del => (KeyCode::Delete, km, kk).into(),
                    NcKey::Backspace => (KeyCode::Backspace, km, kk).into(),
                    NcKey::PgDown => (KeyCode::PageDown, km, kk).into(),
                    NcKey::PgUp => (KeyCode::PageUp, km, kk).into(),
                    NcKey::Home => (KeyCode::Home, km, kk).into(),
                    NcKey::End => (KeyCode::End, km, kk).into(),

                    // - Normal: F1-F12
                    // - +Shift: F13-F24
                    // - +Control: F25-F36
                    // - +Shift+Control: F37-F48
                    NcKey::F00 => (KeyCode::F(0), km, kk).into(),
                    NcKey::F01 => (KeyCode::F(1), km, kk).into(),
                    NcKey::F02 => (KeyCode::F(2), km, kk).into(),
                    NcKey::F03 => (KeyCode::F(3), km, kk).into(),
                    NcKey::F04 => (KeyCode::F(4), km, kk).into(),
                    NcKey::F05 => (KeyCode::F(5), km, kk).into(),
                    NcKey::F06 => (KeyCode::F(6), km, kk).into(),
                    NcKey::F07 => (KeyCode::F(7), km, kk).into(),
                    NcKey::F08 => (KeyCode::F(8), km, kk).into(),
                    NcKey::F09 => (KeyCode::F(9), km, kk).into(),
                    NcKey::F10 => (KeyCode::F(10), km, kk).into(),
                    NcKey::F11 => (KeyCode::F(11), km, kk).into(),
                    NcKey::F12 => (KeyCode::F(12), km, kk).into(),
                    NcKey::F13 => (KeyCode::F(13), km, kk).into(),
                    NcKey::F14 => (KeyCode::F(14), km, kk).into(),
                    NcKey::F15 => (KeyCode::F(15), km, kk).into(),
                    NcKey::F16 => (KeyCode::F(16), km, kk).into(),
                    NcKey::F17 => (KeyCode::F(17), km, kk).into(),
                    NcKey::F18 => (KeyCode::F(18), km, kk).into(),
                    NcKey::F19 => (KeyCode::F(19), km, kk).into(),
                    NcKey::F20 => (KeyCode::F(20), km, kk).into(),
                    NcKey::F21 => (KeyCode::F(21), km, kk).into(),
                    NcKey::F22 => (KeyCode::F(22), km, kk).into(),
                    NcKey::F23 => (KeyCode::F(23), km, kk).into(),
                    NcKey::F24 => (KeyCode::F(24), km, kk).into(),
                    NcKey::F25 => (KeyCode::F(25), km, kk).into(),
                    NcKey::F26 => (KeyCode::F(26), km, kk).into(),
                    NcKey::F27 => (KeyCode::F(27), km, kk).into(),
                    NcKey::F28 => (KeyCode::F(28), km, kk).into(),
                    NcKey::F29 => (KeyCode::F(29), km, kk).into(),
                    NcKey::F30 => (KeyCode::F(30), km, kk).into(),
                    NcKey::F31 => (KeyCode::F(31), km, kk).into(),
                    NcKey::F32 => (KeyCode::F(32), km, kk).into(),
                    NcKey::F33 => (KeyCode::F(33), km, kk).into(),
                    NcKey::F34 => (KeyCode::F(34), km, kk).into(),
                    NcKey::F35 => (KeyCode::F(35), km, kk).into(),
                    NcKey::F36 => (KeyCode::F(36), km, kk).into(),
                    NcKey::F37 => (KeyCode::F(37), km, kk).into(),
                    NcKey::F38 => (KeyCode::F(38), km, kk).into(),
                    NcKey::F39 => (KeyCode::F(39), km, kk).into(),
                    NcKey::F40 => (KeyCode::F(40), km, kk).into(),
                    NcKey::F41 => (KeyCode::F(41), km, kk).into(),
                    NcKey::F42 => (KeyCode::F(42), km, kk).into(),
                    NcKey::F43 => (KeyCode::F(43), km, kk).into(),
                    NcKey::F44 => (KeyCode::F(44), km, kk).into(),
                    NcKey::F45 => (KeyCode::F(45), km, kk).into(),
                    NcKey::F46 => (KeyCode::F(46), km, kk).into(),
                    NcKey::F47 => (KeyCode::F(47), km, kk).into(),
                    NcKey::F48 => (KeyCode::F(48), km, kk).into(),
                    NcKey::F49 => (KeyCode::F(49), km, kk).into(),
                    NcKey::F50 => (KeyCode::F(50), km, kk).into(),
                    NcKey::F51 => (KeyCode::F(51), km, kk).into(),
                    NcKey::F52 => (KeyCode::F(52), km, kk).into(),
                    NcKey::F53 => (KeyCode::F(53), km, kk).into(),
                    NcKey::F54 => (KeyCode::F(54), km, kk).into(),
                    NcKey::F55 => (KeyCode::F(55), km, kk).into(),
                    NcKey::F56 => (KeyCode::F(56), km, kk).into(),
                    NcKey::F57 => (KeyCode::F(57), km, kk).into(),
                    NcKey::F58 => (KeyCode::F(58), km, kk).into(),
                    NcKey::F59 => (KeyCode::F(59), km, kk).into(),
                    NcKey::F60 => (KeyCode::F(60), km, kk).into(),
                    // ... leave room for function keys.
                    NcKey::Enter => (KeyCode::Enter, km, kk).into(),

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
                    NcKey::CapsLock => (KeyCode::CapsLock, km, kk).into(),
                    NcKey::ScrollLock => (KeyCode::ScrollLock, km, kk).into(),
                    NcKey::NumLock => (KeyCode::NumLock, km, kk).into(),
                    NcKey::PrintScreen => (KeyCode::PrintScreen, km, kk).into(),
                    NcKey::Pause => (KeyCode::Pause, km, kk).into(),
                    NcKey::Menu => (KeyCode::Menu, km, kk).into(),

                    // media keys, similarly only available through kitty's protocol
                    NcKey::MediaPlay => (MediaKey::Play, km, kk).into(),
                    NcKey::MediaPause => (MediaKey::Pause, km, kk).into(),
                    NcKey::MediaPPause => (MediaKey::PlayPause, km, kk).into(),
                    NcKey::MediaRev => (MediaKey::Reverse, km, kk).into(),
                    NcKey::MediaStop => (MediaKey::Stop, km, kk).into(),
                    NcKey::MediaFF => (MediaKey::FastForward, km, kk).into(),
                    NcKey::MediaRewind => (MediaKey::Rewind, km, kk).into(),
                    NcKey::MediaNext => (MediaKey::Next, km, kk).into(),
                    NcKey::MediaPrev => (MediaKey::Previous, km, kk).into(),
                    NcKey::MediaRecord => (MediaKey::Record, km, kk).into(),
                    NcKey::MediaLVol => (MediaKey::LowerVolume, km, kk).into(),
                    NcKey::MediaRVol => (MediaKey::RaiseVolume, km, kk).into(),
                    NcKey::MediaMute => (MediaKey::MuteVolume, km, kk).into(),

                    // modifiers when pressed by themselves. this ordering comes from the kitty
                    // keyboard protocol, and mustn't be changed without updating handlers
                    NcKey::LShift => (ModifierKey::LeftShift, km, kk).into(),
                    NcKey::LCtrl => (ModifierKey::LeftControl, km, kk).into(),
                    NcKey::LAlt => (ModifierKey::LeftAlt, km, kk).into(),
                    NcKey::LSuper => (ModifierKey::LeftSuper, km, kk).into(),
                    NcKey::LHyper => (ModifierKey::LeftHyper, km, kk).into(),
                    NcKey::LMeta => (ModifierKey::LeftMeta, km, kk).into(),
                    NcKey::RShift => (ModifierKey::RightShift, km, kk).into(),
                    NcKey::RCtrl => (ModifierKey::RightControl, km, kk).into(),
                    NcKey::RAlt => (ModifierKey::RightAlt, km, kk).into(),
                    NcKey::RSuper => (ModifierKey::RightSuper, km, kk).into(),
                    NcKey::RHyper => (ModifierKey::RightHyper, km, kk).into(),
                    NcKey::RMeta => (ModifierKey::RightMeta, km, kk).into(),
                    // `altgr` in european keyboards
                    NcKey::L3Shift => (ModifierKey::IsoLevel3Shift, km, kk).into(),
                    NcKey::L5Shift => (ModifierKey::IsoLevel5Shift, km, kk).into(),

                    // TODO WIP
                    // mouse events. we encode which button was pressed into the number,
                    // but position information is embedded in the larger ncinput event => KeyCode::None,
                    NcKey::Motion => MouseEvent {
                        button: None,
                        kind: MouseKind::Motion,
                        mods: km,
                        pos: pos.expect("mouse position"),
                        offset: off,
                    }
                    .into(),

                    NcKey::Button1 => {
                        MouseEvent::with_button(1, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button2 => {
                        MouseEvent::with_button(2, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button3 => {
                        MouseEvent::with_button(3, input.itype, km, pos.unwrap(), off).into()
                    }
                    // scrollwheel up
                    NcKey::Button4 => {
                        MouseEvent::with_button(4, MouseKind::ScrollUp, km, pos.unwrap(), off)
                            .into()
                    }
                    // scrollwheel down
                    NcKey::Button5 => {
                        MouseEvent::with_button(5, MouseKind::ScrollDown, km, pos.unwrap(), off)
                            .into()
                    }

                    NcKey::Button6 => {
                        MouseEvent::with_button(6, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button7 => {
                        MouseEvent::with_button(7, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button8 => {
                        MouseEvent::with_button(8, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button9 => {
                        MouseEvent::with_button(9, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button10 => {
                        MouseEvent::with_button(10, input.itype, km, pos.unwrap(), off).into()
                    }
                    NcKey::Button11 => {
                        MouseEvent::with_button(11, input.itype, km, pos.unwrap(), off).into()
                    }

                    // aliases from the 128 characters common to ascii+utf8 => KeyCode::None,
                    NcKey::Tab => (KeyCode::Tab, km, kk).into(),
                    NcKey::Esc => (KeyCode::Esc, km, kk).into(),
                    NcKey::Space => (KeyCode::Space, km, kk).into(),

                    _ => EventKind::None,
                }
            }
            _ => EventKind::None,
        }
    }
}
