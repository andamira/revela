// revela::backend::gilrs
//
//! `gilrs` backend.
//
// TODO
// - https://docs.rs/gilrs/latest/gilrs/struct.Gamepad.html
// - https://docs.rs/gilrs/latest/gilrs/ev/enum.AxisOrBtn.html
// - https://docs.rs/gilrs/latest/gilrs/ev/filter/index.html
// - access state
// - ev::filter::FilterFn,
//
// IMPROVE:
// - support different gamepads
// - support timestamp (convert to EventTimeStamp?)

/// Re-export of the [`gilrs`](https://docs.rs/gilrs) crate.
#[doc(inline)]
pub use ::gilrs;

use crate::all::{
    Backend, Event, EventKind, EventSource, GamepadAxis, GamepadButton, GamepadEvent,
    RevelaError as Error, RevelaResult as Result,
};

use ::gilrs::{Axis, Button, Event as GilrsEvent, EventType, Gilrs, GilrsBuilder};

/// `gilrs` interface.
#[derive(Debug)]
pub struct GilrsBackend {
    inner: Gilrs,
}

impl Backend for GilrsBackend {
    // fn capabilities(&self) -> Capabilities {
    //     self.inner.capabilities().into()
    // }
    fn version(&self) -> (u32, u32, u32) {
        // IMPROVE:detect-version
        (0, 10, 1)
    }
}

impl EventSource for GilrsBackend {
    fn wait_event(&mut self) -> Result<Event> {
        Err(Error::NotSupported)
    }
    fn poll_event(&mut self) -> Result<Event> {
        if let Some(event) = self.inner.next_event() {
            // Ok((event, inner.counter().into()) // MAYBE
            Ok(event.into())
        } else {
            Ok(EventKind::None.into())
        }
    }
}

impl GilrsBackend {
    /// Returns a new gilrs gamepad event source, with default settings
    //
    // https://docs.rs/gilrs/latest/gilrs/struct.GilrsBuilder.html#method.new
    pub fn new() -> Result<Self> {
        let inner = GilrsBuilder::new().build()?;
        // let inner = GilrsBuilder::new().set_update_state(false).build()?;

        Ok(Self { inner })
    }

    //

    /// Creates a new `GilrsBackend` from an existing `gilrs` instance.
    pub fn from_gilrs(gilrs: Gilrs) -> Result<Self> {
        Ok(Self { inner: gilrs })
    }

    /// Increases internal counter by one.
    ///
    /// Counter data is stored with state and can be used to determine when last
    /// event happened. You probably want to use this function in your update
    /// loop after processing events.
    //
    // https://docs.rs/gilrs/latest/gilrs/struct.Gilrs.html#inc
    #[inline]
    pub fn increment(&mut self) {
        self.inner.inc()
    }

    /// Returns the counter.
    ///
    /// Counter data is stored with state and can be used to determine when last
    /// event happened.
    //
    // https://docs.rs/gilrs/latest/gilrs/struct.Gilrs.html#counter
    #[inline]
    pub fn counter(&self) -> u64 {
        self.inner.counter()
    }

    /// Sets the counter to 0.
    #[inline]
    pub fn reset_counter(&mut self) {
        self.inner.reset_counter()
    }
}

impl From<GilrsEvent> for Event {
    fn from(from: GilrsEvent) -> Event {
        GamepadEvent::from(from.event).into()
    }
}
// MAYBE: counter state
// impl From<(GilrsEvent, u64)> for Event {
//     /// Converts a tuple of
//     fn from(tuple: (GilrsEvent, u64)) -> Event {
//         GamepadEvent {
//             kind: tuple.0.event.into(),
//             count: tuple.1,
//             // id: from.id.into(), MAYBE
//             // time: from.time.into()?
//         }
//         .into()
//     }
// }

impl From<EventType> for GamepadEvent {
    fn from(from: EventType) -> GamepadEvent {
        use GamepadEvent::*;
        match from {
            EventType::ButtonPressed(b, _c) => ButtonPressed(b.into()),
            EventType::ButtonRepeated(b, _c) => ButtonRepeated(b.into()),
            EventType::ButtonReleased(b, _c) => ButtonReleased(b.into()),
            EventType::ButtonChanged(b, v, _c) => ButtonChanged(b.into(), v),
            EventType::AxisChanged(a, v, _c) => AxisChanged(a.into(), v),
            EventType::Connected => Connected,
            EventType::Disconnected => Disconnected,
            // (used for filtered out events)
            EventType::Dropped => Dropped,
        }
    }
}

// https://docs.rs/gilrs/latest/gilrs/ev/enum.Button.html
impl From<Button> for GamepadButton {
    fn from(from: Button) -> GamepadButton {
        use GamepadButton::*;
        match from {
            Button::South => South,
            Button::East => East,
            Button::North => North,
            Button::West => West,
            Button::C => C,
            Button::Z => Z,
            Button::LeftTrigger => LeftTrigger,
            Button::LeftTrigger2 => LeftTrigger2,
            Button::RightTrigger => RightTrigger,
            Button::RightTrigger2 => RightTrigger2,
            Button::Select => Select,
            Button::Start => Start,
            Button::Mode => Mode,
            Button::LeftThumb => LeftThumb,
            Button::RightThumb => RightThumb,
            Button::DPadUp => DPadUp,
            Button::DPadDown => DPadDown,
            Button::DPadLeft => DPadLeft,
            Button::DPadRight => DPadRight,
            Button::Unknown => Unknown,
        }
    }
}

// https://docs.rs/gilrs/latest/gilrs/ev/enum.Axis.html
impl From<Axis> for GamepadAxis {
    fn from(from: Axis) -> GamepadAxis {
        use GamepadAxis::*;
        match from {
            Axis::LeftStickX => LeftStickX,
            Axis::LeftStickY => LeftStickY,
            Axis::LeftZ => LeftZ,
            Axis::RightStickX => RightStickX,
            Axis::RightStickY => RightStickY,
            Axis::RightZ => RightZ,
            Axis::DPadX => DPadX,
            Axis::DPadY => DPadY,
            Axis::Unknown => Unknown,
        }
    }
}
