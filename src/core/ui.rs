// revela::base::ui
//
//!
//

pub trait Ui {
    // /// Returns the Ui capabilities.
    // fn capabilities(&self) -> Capabilities;

    // MAYBE
    // /// Allows getting the specific Ui struct from a Ui trait object.
    // fn try_into_backend(self, backend: Backend) -> Option<BackendUi>;

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
