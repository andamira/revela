// revela::core::window
//
//!
//

use crate::all::{Size, UiResult};

/// Comon window trait.
pub trait Window {
    /// Refreshes the window contents.
    //
    // - notcurses:to recalculates its awareness of terminal dimensions.
    //   it is called automatically on render() by the backend.
    // - other backends: check
    fn refresh(&mut self) -> UiResult<()>;

    /// Renders the window contents.
    //
    // - notcurses: renders the root plane. (retained)
    // - other backends: maybe no-op.
    fn render(&mut self) -> UiResult<()>;

    /// Returns the window size in the requested `unit`, if possible.
    fn size(&self) -> Size;
}
