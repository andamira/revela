// revela::core::capabilities::window
//
//!
//
// IMPROVE
// - size
// - events, controllable…?

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowCapabilities {
    /// Whether multiple windows are supported.
    pub multi: bool,
}
