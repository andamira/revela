// revela::core::capabilities::text_grid
//
//!
//
// IMPROVE
// - suported blitters

use crate::all::Size;

/// Text-grid capabilities.
//
// - https://docs.rs/notcurses/latest/notcurses/struct.Capabilities.html
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextGridCapabilities {
    /* dimensions */
    // /// Maximum grid size.
    // pub max_grid_size: Option<Size>,
    /// Predetermined cell size, in native pixels, if known.
    //
    // crossterm: unknown
    // notcurses: known (almost always?)
    pub cell_size: Option<Size>,

    /// Whether the cell size can be customized.
    //
    // only supported in custom implementations over a canvas,
    // or maybe when setting the terminal font-size programatically is possible.
    pub custom_cell_size: bool,

    /* glyphs */
    /// Whether unicode is supported.
    pub unicode: bool,
    // /// Supported blitters
    // pub blitters: ...,

    // /// Supported styles
    // pub styles: ...,
}
