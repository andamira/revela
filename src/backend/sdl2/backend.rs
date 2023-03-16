// revela::backend::sdl2::backend
//
//! Backend wrapper for `sdl2`
//

use crate::all::{
    Backend, Capabilities, ColorCapabilities, InputCapabilities, PixelCapabilities,
    RevelaResult as Result, SoundCapabilities, WindowCapabilities,
};
use sdl2::{init, Sdl};

/// `sdl2` [`Backend`].
pub struct Sdl2Backend {
    pub(crate) sdl: Sdl,
}

/// # constructors
impl Sdl2Backend {
    /// Creates a new Sdl backend.
    pub fn new() -> Result<Self> {
        Ok(Self { sdl: init()? })
    }
}

/// # methods
impl Sdl2Backend {}

mod std_impls {
    use super::{Backend, Sdl, Sdl2Backend};
    use std::fmt;

    impl fmt::Debug for Sdl2Backend {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Sdl2Backend {{ {} }}", self.version_string(),)
        }
    }

    impl From<Sdl> for Sdl2Backend {
        fn from(sdl: Sdl) -> Sdl2Backend {
            Sdl2Backend { sdl }
        }
    }
    impl From<Sdl2Backend> for Sdl {
        fn from(sdl: Sdl2Backend) -> Sdl {
            sdl.sdl
        }
    }
}

impl Backend for Sdl2Backend {
    fn capabilities(&self) -> Capabilities {
        let color = Some(ColorCapabilities {
            rgb: true,
            ..Default::default()
        });

        let input = Some(InputCapabilities {
            keyboard: true,
            mouse: true,
            ..Default::default()
        });

        let pixel = Some(PixelCapabilities {
            max_bitmap_size: None,
            pixel_native: true,
            // ..Default::default()
        });

        // IMPROVE: devices available, names…
        let sound = Some(SoundCapabilities { sound: true });

        // IMPROVE hidden
        let window = Some(WindowCapabilities { multi: true });

        Capabilities {
            color,
            input,
            pixel,
            sound,
            window,
            ..Default::default()
        }
    }
    fn version(&self) -> (u32, u32, u32) {
        let v = sdl2::version::version();
        (v.major as u32, v.minor as u32, v.patch as u32)
    }
    // fn try_into_backend(self, backend: Backend) -> Option<BackendElement> {
    //     match backend {
    //         Backend::Sdl => Some(BackendElement::Sdl2Backend(self)),
    //         _ => None,
    //     }
    // }
}
