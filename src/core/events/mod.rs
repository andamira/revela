// revela::core::events
//
//! Event related types and traits.
//

use crate::all::UiResult;

// mod gamepad;
pub mod keyboard;
// mod mouse;
pub mod window;
// pub use gamepad::{GamepadEvent, GamepadEventKind, GamepadButton, GamepadAxis};
#[doc(inline)]
pub use keyboard::{Code, KeyEvent, KeyModifiers, MediaKey, ModifierKey};
// pub use mouse::{MouseEvent, MouseButton};
#[doc(inline)]
pub use window::WindowEvent;

/// A source of events.
pub trait EventSource {
    /// Waits for an event, blocking.
    fn wait_event(&mut self) -> UiResult<Event>;

    // /// Waits for an event blocking for the provided `duration`.
    // MAYBE
    // fn wait_event_for(&mut self, duration: Duration) -> UiResult<BackendEvent>;

    /// Polls for an event, non-blocking.
    fn poll_event(&mut self) -> UiResult<Event>;
}

// MAYBE DESIGN
// pub struct Event {
//     source: Backends,
//     // THINK
//     // time: Timestamp32,
// }

/// A an enumeration of events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
//   - https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs
// - https://docs.rs/notcurses/latest/notcurses/struct.Input.html
//   - https://docs.rs/notcurses/latest/notcurses/enum.Received.html
// - https://docs.rs/sdl2/latest/sdl2/event/enum.Event.html
//
#[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub enum EventData {
pub enum Event {
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A window event.
    Window(WindowEvent),

    /// A keyboard event.
    Key(KeyEvent),
    // /// A mouse event.
    // // WIP
    // Mouse(MouseEvent),

    // /// A gamepad event.
    // // WIP
    // Gamepad(GamepadEvent),

    // /// A midi event.
    // // TODO
    // Midi(MidiEvent),

    // crossterm, maybe other
    // Paste(String),
}

impl Event {
    /// Returns `true` if it's some event.
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        !matches![self, Event::None]
    }
    /// Returns `true` if there's no event.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        matches![self, Event::None]
    }

    /// Returns `true` if it's a window event.
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        matches![self, Event::Window(_)]
    }

    /// Returns `true` if it's a keyboard event.
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        matches![self, Event::Key(_)]
    }

    // /// Returns `true` if it's a mouse event.
    // #[inline(always)]
    // pub fn is_mouse(&self) -> bool {
    //     matches![self, Event::Mouse(_)]
    // }

    // /// Returns `true` if it's a mouse event.
    // #[inline(always)]
    // pub fn is_gamepad(&self) -> bool {
    //     matches![self, Event::Gamepad(_)]
    // }

    //

    // /// Returns `true` if it's a press kind of event.
    // #[inline(always)]
    // pub fn is_press(&self) -> bool {
    //     // matches![self, Event::Gamepad(_)]
    //     todo![]
    // }
}
