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

    /// Returns the window size, in the appropriate units for the backend.
    fn size(&self) -> Result<Size>;

    /// Tries to set the window size, in the appropriate units for the backend.
    fn set_size(&mut self, size: Size) -> Result<()>;
}
