// revela::core::capabilities::pixel
//
//!
//
//

use crate::all::Size;

///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelCapabilities {
    /// Maximum bitmap size, in native pixels.
    max_bitmap_size: Option<Size>,
}
