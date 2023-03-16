// revela::capabilities::win
//
//!
//

/// Input capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputCapabilities {
    ///
    pub gamepad: bool,
    ///
    pub keyboard: bool,
    ///
    pub midi: bool,
    ///
    pub mouse: bool,
}
