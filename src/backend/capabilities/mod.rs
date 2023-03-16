// revela::capabilities
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
// mod terminal;
mod text_grid;
mod window;

pub use color::ColorCapabilities;
pub use input::InputCapabilities;
pub use pixel::PixelCapabilities;
pub use sound::SoundCapabilities;
#[cfg(feature = "std")] // IMPROVE: alloc
pub use system::SystemCapabilities;
// pub use terminal::TerminalCapabilities;
pub use text_grid::TextGridCapabilities;
pub use window::WindowCapabilities;

/// The capabilities supported by a backend.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Capabilities {
    pub color: Option<ColorCapabilities>,
    pub input: Option<InputCapabilities>,
    pub pixel: Option<PixelCapabilities>,
    pub text_grid: Option<TextGridCapabilities>,
    pub sound: Option<SoundCapabilities>,
    #[cfg(feature = "std")] // IMPROVE: alloc
    pub system: Option<SystemCapabilities>,

    // ///
    // pub terminal: Option<TerminalCapabilities>,
    pub window: Option<WindowCapabilities>,
}

impl Capabilities {
    // IMPROVE:FIX
    // pub fn best<I: Iterator<Inner: C>, C: Into<Capabilities>>() -> Capabilities

    // /// Returns the best `Capabilities` from a slice.
    // pub fn best<C: Into<Capabilities>>(slice: &[C]) -> Capabilities {
    //     todo![]
    //
    //     // TODO: apply sum()
    //     // let sum = slice.iter().map(|x|x);
    //     // Self::default() // TEMP
    // }

    /// Returns
    pub fn sum() {
        todo![]
    }

    // /// Returns the best Capabilities from an iterator.
    // TODO asiste
    // pub fn best<I: Iterator<Inner: C>, C: Into<Capabilities>>() -> Capabilities
    //     where I<Inner> = C { Self::default() // TEMP
    // }
}
