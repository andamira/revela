// revela::core::capabilities
//
//! backend capabilities.
//
// IMPROVE
// - AccesibilityCapabilities

mod color;
mod input;
mod pixel;
mod sound;
#[cfg(feature = "std")] // IMPROVE: alloc
mod system;
mod text_grid;
mod window;

pub use color::ColorCapabilities;
pub use input::InputCapabilities;
pub use pixel::PixelCapabilities;
pub use sound::SoundCapabilities;
#[cfg(feature = "std")] // IMPROVE: alloc
pub use system::SystemCapabilities;
pub use text_grid::TextGridCapabilities;
pub use window::WindowCapabilities;

/// The capabilities supported by a backend.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Capabilities {
    ///
    pub color: Option<ColorCapabilities>,
    ///
    pub input: Option<InputCapabilities>,
    ///
    pub pixel: Option<PixelCapabilities>,
    ///
    pub text_grid: Option<TextGridCapabilities>,
    ///
    pub sound: Option<SoundCapabilities>,
    ///
    #[cfg(feature = "std")] // IMPROVE: alloc
    pub system: Option<SystemCapabilities>,
    ///
    pub window: Option<WindowCapabilities>,
}
