// revela::backend::backend
//
//!
//

use crate::backend::Capabilities;

pub trait Backend {
    /// Returns the Backend capabilities.
    fn capabilities(&self) -> Capabilities;

    // MAYBE
    // /// Allows getting the specific Backend struct from a Backend trait object.
    // fn try_into_backend(self, backend: Backend) -> Option<Backends>;

    /// Returns the inner backend version numbers (major, minor, patch).
    fn version(&self) -> (u32, u32, u32);

    /* auto impls */

    /// Returns the backend version string.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn version_string(&self) -> String {
        let v = self.version();
        format!["v{}.{}.{}", v.0, v.1, v.2]
    }
}
