// revela::core::capabilities::color
//
//!
//
// NOTE: maybe say backend support, not necessaryly terminal support
// meaning, if we're not sure of the current terminal, say yes, regardless...
// preferences can further limit a capability that doesn't work well
// under current circumstances (e.g. specific terminal).

/// Color-related capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ColorCapabilities {
    /// Whether it's possible to specify rgb values.
    pub rgb: bool,

    ///
    pub palette: Option<u16>,

    pub palette_change: bool,
    pub palette_size: u16,
}
