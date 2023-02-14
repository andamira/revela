// revela::core::capabilities::text_grid
//
//!
//

use crate::all::Size;

/// Text-grid capabilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextGridCapabilities {
    /// Maximum grid size.
    max_grid_size: Size,
    /// Predetermined cell size, in native pixels.
    cell_size: Size,
    /// Whether the cell size can be customized.
    custom_cell_size: bool,

    // /// Supported styles
    // styles
    ///
    unicode: bool,
}

impl TextGridCapabilities {}
