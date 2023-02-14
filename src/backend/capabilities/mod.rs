// revela::core::capabilities
//
//!
//

mod pixel;
mod text_grid;

pub use pixel::PixelCapabilities;
pub use text_grid::TextGridCapabilities;

/// The supported capabilities of the UI backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Capabilities {
    pub text_grid: Option<TextGridCapabilities>,

    pub pixel: Option<PixelCapabilities>,
}
