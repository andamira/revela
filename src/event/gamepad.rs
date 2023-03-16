// revela::event::gamepad
//
//! Gamepad events.
//

use crate::all::{Event, EventKind};

// /// A gamepad event.
// // WIP
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct GamepadEvent {
//     pub kind: GamepadEventKind,
//     // pub id: GamepadId,
//     // pub id: usize, // RETHINK
//     // pub event: EventType,
//     // pub time: SystemTime, // -> to Event struct timestamp field (but monotonic‥)
// }

impl From<GamepadEvent> for EventKind {
    fn from(gamepad_event: GamepadEvent) -> EventKind {
        EventKind::Gamepad(gamepad_event)
    }
}
impl From<GamepadEvent> for Event {
    fn from(gamepad_event: GamepadEvent) -> Event {
        EventKind::from(gamepad_event).into()
    }
}

// MAYBE:
// ///
// // TODO: DESIGN: Depending on source…
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// #[non_exhaustive]
// // https://docs.rs/gilrs/latest/gilrs/struct.GamepadId.html
// // impls to usize
// pub enum GamepadId {
//     #[cfg(feature = "gilrs")]
//     Gilrs(gilrs::GamepadId)
// }

///
// WIP
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum GamepadEvent {
    ///
    // ButtonPressed(GamepadButton, Code),
    ButtonPressed(GamepadButton),
    ///
    // ButtonReleased(GamepadButton, Code),
    ButtonReleased(GamepadButton),
    // ///
    // ButtonChanged(GamepadButton, f32, Code),
    ButtonChanged(GamepadButton, f32), // MAYBE? int/bool
    ///
    // ButtonRepeated(GamepadButton, Code),
    ButtonRepeated(GamepadButton),
    // AxisChanged(GamepadAxis, f32, Code),
    AxisChanged(GamepadAxis, f32),
    ///
    Connected,
    ///
    Disconnected,
    ///
    Dropped,
}

///
// https://docs.rs/gilrs/latest/gilrs/ev/enum.Button.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GamepadButton {
    // 8bitdo=A
    South,
    // 8bitdo=B
    East,
    // 8bitdo=X
    North,
    // 8bitdo=Y
    West,
    C,
    Z,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Unknown,
}

///
// https://docs.rs/gilrs/latest/gilrs/ev/enum.Axis.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
    Unknown,
}
