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
    /// Returns a new mouse event.
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

    /// Returns `true` if it's a mouse press.
    #[inline(always)]
    pub fn is_press(&self) -> bool {
        self.kind.is_press()
    }

    /// Returns `true` if it's a mouse release.
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        self.kind.is_release()
    }

    /// Returns `true` if it's a mouse motion.
    #[inline(always)]
    pub fn is_motion(&self) -> bool {
        self.kind.is_motion()
    }

    /// Returns `true` if it's a mouse scroll.
    #[inline(always)]
    pub fn is_scroll(&self) -> bool {
        self.kind.is_scroll() || self.button.map_or(false, |b| b.is_scroll())
    }

    /// Returns `true` if it's a mouse scroll up.
    #[inline(always)]
    pub fn is_scroll_up(&self) -> bool {
        self.kind.is_scroll_up() || self.button.map_or(false, |b| b.is_scroll_up())
    }

    /// Returns `true` if it's a mouse scroll down.
    #[inline(always)]
    pub fn is_scroll_down(&self) -> bool {
        self.kind.is_scroll_down() || self.button.map_or(false, |b| b.is_scroll_down())
    }

    /// Returns `true` if it's a left button.
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        self.button.map_or(false, |b| b.is_left())
    }
    /// Returns `true` if it's a middle button.
    #[inline(always)]
    pub fn is_middle(&self) -> bool {
        self.button.map_or(false, |b| b.is_middle())
    }
    /// Returns `true` if it's a right button.
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        self.button.map_or(false, |b| b.is_right())
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
    ScrollUp,
    ScrollDown,
}
impl MouseKind {
    /// Returns `true` if it's a mouse press.
    #[inline(always)]
    pub fn is_press(&self) -> bool {
        matches![self, MouseKind::Press]
    }

    /// Returns `true` if it's a mouse release.
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        matches![self, MouseKind::Release]
    }

    /// Returns `true` if it's a mouse motion.
    #[inline(always)]
    pub fn is_motion(&self) -> bool {
        matches![self, MouseKind::Motion]
    }

    /// Returns `true` if it's a mouse scroll.
    #[inline(always)]
    pub fn is_scroll(&self) -> bool {
        use MouseKind::*;
        matches![self, ScrollUp | ScrollDown]
    }

    /// Returns `true` if it's a mouse scroll up.
    #[inline(always)]
    pub fn is_scroll_up(&self) -> bool {
        matches![self, MouseKind::ScrollUp]
    }

    /// Returns `true` if it's a mouse scroll down.
    #[inline(always)]
    pub fn is_scroll_down(&self) -> bool {
        matches![self, MouseKind::ScrollDown]
    }
}

/// Mouse buttons.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.MouseButton.html
// - https://docs.rs/sdl2/latest/sdl2/mouse/enum.MouseButton.html
//   Unknown, Left, Middle, Right, X1, X2,
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MouseButton {
    /// Left mouse button.
    Left = 1,

    /// Middle mouse button.
    Middle = 2,

    /// Right mouse button.
    Right = 3,

    /// The scroll up mouse wheel button.
    ScrollUp = 4,

    /// The scroll down mouse wheel button.
    ScrollDown = 5,

    /// Other mouse button.
    Other(u8),
}

impl MouseButton {
    /// Returns `true` if it's a left button.
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        matches![self, MouseButton::Left]
    }
    /// Returns `true` if it's a middle button.
    #[inline(always)]
    pub fn is_middle(&self) -> bool {
        matches![self, MouseButton::Middle]
    }
    /// Returns `true` if it's a right button.
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        matches![self, MouseButton::Right]
    }
    /// Returns `true` if it's a scroll wheel button.
    #[inline(always)]
    pub fn is_scroll(&self) -> bool {
        use MouseButton::*;
        matches![self, ScrollUp | ScrollDown]
    }
    /// Returns `true` if it's a scroll up wheel button.
    #[inline(always)]
    pub fn is_scroll_up(&self) -> bool {
        matches![self, MouseButton::ScrollUp]
    }
    /// Returns `true` if it's a scroll down wheel button.
    #[inline(always)]
    pub fn is_scroll_down(&self) -> bool {
        matches![self, MouseButton::ScrollDown]
    }
}

impl From<u8> for MouseButton {
    fn from(b: u8) -> MouseButton {
        use MouseButton::*;
        match b {
            1 => Left,
            2 => Middle,
            3 => Right,
            4 => ScrollUp,
            5 => ScrollDown,
            _ => Other(b),
        }
    }
}
