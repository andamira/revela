// revela::core::events::window
//
//! Window events.
//

use crate::all::{Event, EventKind, Size};

/// Events related to the [`Window`][crate::all::Window].
//
// - https://docs.rs/sdl2/latest/sdl2/event/enum.WindowEvent.html
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowEvent {
    FocusGained,
    FocusLost,
    // Resized,
    // crossterm and sdl has this, MAYBE add for notcurses
    Resized(Option<Size>),
    // NOTE the difference in SDL: https://stackoverflow.com/a/55076700/940200
    // SizeChanged(Size),
    /// Signal to continue (SIGCONT).
    ///
    /// This typically indicates that the program has been restarted after being
    /// paused or placed in the background.
    //
    // https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_signal
    Continue,

    /// Signal of end of input.
    ///
    /// Notcurses specific. [see doc][0].
    ///
    /// [0]: https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_eof
    EndOfInput,

    /// Paste action.
    Paste(String),
    // Moved(Position),

    // Shown,
    // Hidden,
    // Exposed,
    // Minimized,
    // Maximized,
    // Restored,
    // Enter,
    // Leave,
    // Close,
    // TakeFocus,
    // HitTest,
}
impl From<WindowEvent> for EventKind {
    fn from(window_event: WindowEvent) -> EventKind {
        EventKind::Window(window_event)
    }
}
impl From<WindowEvent> for Event {
    fn from(window_event: WindowEvent) -> Event {
        EventKind::from(window_event).into()
    }
}
