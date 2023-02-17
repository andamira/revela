// revela::core::window
//
//!
//

use crate::all::{RevelaResult as Result, Size};

/// Window trait.
pub trait Window {
    /// Refreshes the window contents.
    //
    // - notcurses:to recalculates its awareness of terminal dimensions.
    //   it is called automatically on render() by the backend.
    // - other backends: check
    fn refresh(&mut self) -> Result<()>;

    // MAYBE
    // fn raster(&mut self) -> Result<()>;

    /// Renders the window contents.
    //
    // - notcurses: renders the root plane. (retained)
    // - other backends: maybe no-op.
    fn render(&mut self) -> Result<()>;

    // RETHINK
    /// Returns the window size, in native pixels if possible.
    fn size(&self) -> Size;
}
