// revela::capabilities::system
//
//!
//

/// System capabilities.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemCapabilities {
    /// The name of the detected OS version.
    pub os_version: Option<String>,

    ///
    pub user_name: Option<String>,
    ///
    pub host_name: Option<String>,

    /// The name of the system.
    ///
    /// E.g. the terminal name.
    // IMPROVE:
    // implementation details?
    pub terminal_name: Option<String>,
}
