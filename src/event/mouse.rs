// revela::event::mouse
//
//! Mouse events.
//

use crate::all::{Event, EventKind, KeyModifiers, Position};

/// Mouse events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/struct.MouseEvent.html
// - https://docs.rs/sdl2/latest/sdl2/mouse/index.html
// - https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent
// - https://docs.rs/sfml/latest/sfml/window/enum.Event.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MouseEvent {
    // THINK here or as part of MouseKind
    pub button: Option<MouseButton>,
    pub kind: MouseKind,
    pub mods: KeyModifiers,

    // option A)
    // pub cell: Option<Position>,
    // pub pixel: Option<Position>,

    // option B)
    pub pos: Position,

    /// Optional offset for the position.
    ///
    /// This can be used to give a pixel offset over a cell position in a grid of cells.
    pub offset: Option<Position>,
}

impl MouseEvent {
    /// Returns a new
    pub fn with_button(
        button: impl Into<MouseButton>,
        kind: impl Into<MouseKind>,
        mods: KeyModifiers,
        pos: Position,
        offset: Option<Position>,
    ) -> Self {
        MouseEvent {
            button: Some(button.into()),
            kind: kind.into(),
            mods,
            pos,
            offset,
        }
    }
}

impl From<MouseEvent> for EventKind {
    #[inline]
    fn from(mouse_event: MouseEvent) -> EventKind {
        EventKind::Mouse(mouse_event)
    }
}

impl From<MouseEvent> for Event {
    #[inline]
    fn from(mouse_event: MouseEvent) -> Event {
        EventKind::Mouse(mouse_event).into()
    }
}

/// The kind of mouse event.
//
//   - https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseKind {
    Press,
    Release,
    Motion,
    // button 4
    ScrollUp,
    // button 5
    ScrollDown,
}

/// Mouse buttons.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.MouseButton.html
// - https://docs.rs/sdl2/latest/sdl2/mouse/enum.MouseButton.html
//   Unknown, Left, Middle, Right, X1, X2,
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    /// Includes 4 for ScrollUp & 5 for ScrollDown
    Other(u8),
}

impl From<u8> for MouseButton {
    fn from(b: u8) -> MouseButton {
        use MouseButton::*;
        match b {
            1 => Left,
            3 => Right,
            2 => Middle,
            _ => Other(b),
        }
    }
}
