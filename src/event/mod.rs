// revela::event
//
//! Event related types and traits.
//

use crate::all::RevelaResult as Result;

mod event;
pub mod gamepad;
pub mod keys;
pub mod midi;
pub mod mouse;
mod time;
mod window;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        event::{Event, EventKind},
        gamepad::{GamepadAxis, GamepadButton, GamepadEvent},
        keys::{FunctionKey, KeyCode, KeyEvent, KeyKind, KeyModifiers, MediaKey, ModifierKey},
        midi::{
            MidiChannel, MidiControl, MidiEvent, MidiFrame, MidiNote, MidiProgram, MidiValue14,
            MidiValue7,
        },
        mouse::{MouseButton, MouseEvent, MouseKind},
        time::EventTimeStamp,
        window::WindowEvent,
        EventSource,
    };
}

/// A source of events.
pub trait EventSource {
    /// Waits for an event, blocking.
    fn wait_event(&mut self) -> Result<Event>;

    // MAYBE
    // /// Waits for an event blocking for the provided `duration`.
    // fn wait_event_for(&mut self, duration: Duration) -> Result<BackendEvent>;

    // MAYBE
    // ///
    // fn poll_events(&mut self, duration: Duration) -> Result<BackendEvent>;

    /// Polls for an event, non-blocking.
    fn poll_event(&mut self) -> Result<Event>;
}
