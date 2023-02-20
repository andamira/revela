// revela::core::capabilities::pixel
//
//!
//
//

use crate::all::Size;

/// Pixel-related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PixelCapabilities {
    /// Maximum bitmap size, in native pixels.
    pub max_bitmap_size: Option<Size>,

    /// Whether pixel-accurate bitmaps are supported.
    pub pixel_native: bool,
}
