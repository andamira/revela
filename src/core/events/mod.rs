// revela::core::events
//
//! Event related types and traits.
//

use crate::all::RevelaResult as Result;

pub mod gamepad;
pub mod keyboard;
pub mod midi;
// pub mod mouse;
mod time;
mod window;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        gamepad::{GamepadAxis, GamepadButton, GamepadEvent},
        keyboard::{KeyCode, KeyEvent, KeyModifiers, MediaKey, ModifierKey},
        midi::{
            MidiChannel, MidiControl, MidiEvent, MidiFrame, MidiNote, MidiProgram, MidiValue14,
            MidiValue7,
        },
        // pub use mouse::{MouseButton, MouseEvent},
        time::EventTimeStamp,
        window::WindowEvent,
        Event,
        EventKind,
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

/// An event.
#[derive(Clone, Copy)]
pub struct Event {
    pub kind: EventKind,
    // midir
    pub emitted: Option<EventTimeStamp>,
    // TODO
    // processed: Option<EventTimeStamp>, // revela
    // count: Option<EventCounter>, // gilrs
}
impl Event {
    /// A `None` event.
    #[allow(non_upper_case_globals)]
    pub const None: Event = Event {
        kind: EventKind::None,
        emitted: None,
    };

    /// Returns a new event.
    pub fn new(kind: EventKind, emitted: Option<EventTimeStamp>) -> Event {
        Self { kind, emitted }
    }
}
impl From<EventKind> for Event {
    fn from(kind: EventKind) -> Event {
        Self {
            kind,
            emitted: None,
        }
    }
}

/// A an enumeration of kinds of events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
//   - https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs
// - https://docs.rs/notcurses/latest/notcurses/struct.Input.html
//   - https://docs.rs/notcurses/latest/notcurses/enum.Received.html
// - https://docs.rs/sdl2/latest/sdl2/event/enum.Event.html
//
#[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub enum EventData {
pub enum EventKind {
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A window event.
    Window(WindowEvent),

    /// A keyboard event.
    Key(KeyEvent),

    /// A midi event.
    Midi(MidiEvent),

    // /// A mouse event.
    // Mouse(MouseEvent),
    /// A gamepad event.
    Gamepad(GamepadEvent),
    // crossterm, maybe other
    // Paste(String),
}

impl EventKind {
    /// Returns `true` if it's some event.
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        !matches![self, EventKind::None]
    }
    /// Returns `true` if there's no event.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        matches![self, EventKind::None]
    }

    /// Returns `true` if it's a window event.
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        matches![self, EventKind::Window(_)]
    }

    /// Returns `true` if it's a keyboard event.
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        matches![self, EventKind::Key(_)]
    }

    /// Returns `true` if it's a keyboard event.
    #[inline(always)]
    pub fn is_midi(&self) -> bool {
        matches![self, EventKind::Midi(_)]
    }

    // /// Returns `true` if it's a mouse event.
    // #[inline(always)]
    // pub fn is_mouse(&self) -> bool {
    //     matches![self, EventKind::Mouse(_)]
    // }

    /// Returns `true` if it's a mouse event.
    #[inline(always)]
    pub fn is_gamepad(&self) -> bool {
        matches![self, EventKind::Gamepad(_)]
    }

    //

    // /// Returns `true` if it's a press kind of event.
    // #[inline(always)]
    // pub fn is_press(&self) -> bool {
    //     // matches![self, Event::Gamepad(_)]
    //     todo![]
    // }
}
