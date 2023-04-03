// revela::event::event
//
//! Define `Event` & `EventKind`.
//

use super::{EventTimeStamp, GamepadEvent, KeyEvent, MidiEvent, MouseEvent, WindowEvent};

/// An event.
#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    // midir
    pub emitted: Option<EventTimeStamp>,
    // processed: Option<EventTimeStamp>, // revela
    // count: Option<EventCounter>, // gilrs

    // TODO: add original backend event, with optional state.
    // NOTE: maybe implies this has to be generic…
    // pub orig: Option<>,
    // pub data: Option<>,
}
impl Event {
    /// A `None` event.
    pub const None: Event = Event {
        kind: EventKind::None,
        emitted: None,
    };

    /// Return a new event with a `kind` and an optional `emmitted` timestamp.
    #[inline]
    pub fn new(kind: EventKind, emitted: Option<EventTimeStamp>) -> Event {
        Self { kind, emitted }
    }

    //

    /// Returns some timestamp of the moment the event was emitted,
    /// or `None` if it's unknown.
    pub fn emitted(&self) -> Option<EventTimeStamp> {
        self.emitted
    }

    /// Returns the kind of event.
    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    /// Returns `true` if there's no event.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.kind.is_none()
    }
    /// Returns `true` if it's some event.
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        self.kind.is_some()
    }

    /// Returns true if it's a window event.
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        self.kind.is_window()
    }
    /// Returns true if it's a keyboard event.
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        self.kind.is_key()
    }
    /// Returns true if it's a mouse event.
    #[inline(always)]
    pub fn is_mouse(&self) -> bool {
        self.kind.is_mouse()
    }
    /// Returns true if it's a gamepad event.
    #[inline(always)]
    pub fn is_gamepad(&self) -> bool {
        self.kind.is_gamepad()
    }
    /// Returns true if it's a midi event.
    #[inline(always)]
    pub fn is_midi(&self) -> bool {
        self.kind.is_midi()
    }

    //

    /// Returns some window event, if that's the kind.
    #[inline(always)]
    pub fn some_window(&self) -> Option<&WindowEvent> {
        self.kind.some_window()
    }

    /// Returns some keyboard event, if that's the kind.
    #[inline(always)]
    #[rustfmt::skip]
    pub fn some_key(&self) -> Option<&KeyEvent> {
        self.kind.some_key()
    }

    /// Returns some mouse event, if that's the kind.
    #[inline(always)]
    pub fn some_mouse(&self) -> Option<&MouseEvent> {
        self.kind.some_mouse()
    }

    /// Returns some gamepad event, if that's the kind.
    #[inline(always)]
    pub fn some_gamepad(&self) -> Option<&GamepadEvent> {
        self.kind.some_gamepad()
    }

    /// Returns some midi event, if that's the kind.
    #[inline(always)]
    pub fn some_midi(&self) -> Option<&MidiEvent> {
        self.kind.some_midi()
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
#[derive(Clone, Debug, Default, PartialEq)]
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

    /// A mouse event.
    Mouse(MouseEvent),

    /// A gamepad event.
    Gamepad(GamepadEvent),
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

    /// Returns `true` if it's a mouse event.
    #[inline(always)]
    pub fn is_mouse(&self) -> bool {
        matches![self, EventKind::Mouse(_)]
    }

    /// Returns `true` if it's a mouse event.
    #[inline(always)]
    pub fn is_gamepad(&self) -> bool {
        matches![self, EventKind::Gamepad(_)]
    }

    /// Returns some window event, if that's the kind.
    #[inline]
    #[rustfmt::skip]
    pub fn some_window(&self) -> Option<&WindowEvent> {
        if let EventKind::Window(e) = &self { Some(e) } else { None }
    }

    /// Returns some keyboard event, if that's the kind.
    #[inline]
    #[rustfmt::skip]
    pub fn some_key(&self) -> Option<&KeyEvent> {
        if let EventKind::Key(e) = &self { Some(e) } else { None }
    }

    /// Returns some mouse event, if that's the kind.
    #[inline]
    #[rustfmt::skip]
    pub fn some_mouse(&self) -> Option<&MouseEvent> {
        if let EventKind::Mouse(e) = &self { Some(e) } else { None }
    }

    /// Returns some gamepad event, if that's the kind.
    #[inline]
    #[rustfmt::skip]
    pub fn some_gamepad(&self) -> Option<&GamepadEvent> {
        if let EventKind::Gamepad(e) = &self { Some(e) } else { None }
    }

    /// Returns some midi event, if that's the kind.
    #[inline]
    #[rustfmt::skip]
    pub fn some_midi(&self) -> Option<&MidiEvent> {
        if let EventKind::Midi(e) = &self { Some(e) } else { None }
    }
}
