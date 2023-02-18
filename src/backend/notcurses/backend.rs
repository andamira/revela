// revela::backend::notcurses::backend
//
//! Backend wrapper for `notcurses`
//
// TODO: Capabilities, Code, Unit

use super::NotcursesTextGrid;
use notcurses::{Notcurses, Plane};

use crate::all::{
    Backend, Event, EventSource, RevelaError as Error, RevelaResult as Result, Size, TextGrid,
    Window, Zone,
};

/// `notcurses` interface.
///
/// It implements the following traits: [`Backend`], [`Window`], [`EventSource`].
pub struct NotcursesBackend {
    inner: Notcurses,

    /// the root text grid, (should be the *cli* plane).
    root: NotcursesTextGrid,
}

impl NotcursesBackend {
    /// Creates a new `NotcursesBackend`.
    pub fn new() -> Result<Self> {
        let mut inner = Notcurses::new()?;
        let root = NotcursesTextGrid::from_plane(inner.cli_plane()?);
        Ok(Self { inner, root })
    }

    /// Returns a shared reference to the root text grid of the window.
    pub fn ref_root(&self) -> &NotcursesTextGrid {
        &self.root
    }

    /// Returns an exclusive reference to the root text grid of the window.
    pub fn mut_root(&mut self) -> &mut NotcursesTextGrid {
        &mut self.root
    }

    pub fn new_root_child(&mut self, zone: impl Into<Zone>) -> Result<NotcursesTextGrid> {
        self.root.new_child(zone)
    }

    //

    /// Creates a new `NotcursesBackend` from an existing `notcurses` instance,
    /// and an optional `root` [`Plane`].
    ///
    /// When no plane is given, it will try to instantiate the *cli* plane.
    ///
    /// # Errors
    /// Returns an error if the CLI plane has been already instantiated.
    pub fn from_notcurses(nc: Notcurses, root: Option<Plane>) -> Result<Self> {
        let mut inner = nc;
        let root = NotcursesTextGrid::from_plane(
            // inner.cli_plane()?
            if let Some(plane) = root {
                plane
            } else {
                inner.cli_plane()?
            },
        );
        Ok(Self { inner, root })
    }

    pub fn into_inner(self) -> Notcurses {
        self.inner
    }
    pub fn ref_inner(&self) -> &Notcurses {
        &self.inner
    }
    pub fn mut_inner(&mut self) -> &mut Notcurses {
        &mut self.inner
    }
}

impl NotcursesBackend {
    /// Enables receiving mouse events.
    pub fn enable_mouse(&mut self) -> Result<()> {
        Ok(self.inner.mice_enable(::notcurses::MiceEvents::All)?)
    }
    /// Disables receiving mouse events.
    pub fn disable_mouse(&mut self) -> Result<()> {
        Ok(self.inner.mice_disable()?)
    }
}

impl Backend for NotcursesBackend {
    // TODO
    // fn capabilities(&self) -> Capabilities {
    //     self.inner.capabilities().into()
    // }

    fn version(&self) -> (u32, u32, u32) {
        let v = Notcurses::version_components();
        (v.0, v.1, v.2)
    }

    // fn try_into_backend(self, backend: Backends) -> Option<BackendElement> {
    //     match backend {
    //         Backends::Notcurses => Some(BackendElement::NotcursesBackend(self)),
    //         #[allow(unreachable_patterns)] // TEMP
    //         _ => None,
    //     }
    // }
}

impl EventSource for NotcursesBackend {
    fn wait_event(&mut self) -> Result<Event> {
        Ok(self.inner.get_event()?.into())
    }

    fn poll_event(&mut self) -> Result<Event> {
        Ok(self.inner.poll_event()?.into())
    }
}

impl Window for NotcursesBackend {
    fn refresh(&mut self) -> Result<()> {
        let _ = self.inner.refresh()?;
        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        self.root.render()?;
        Ok(())
    }

    fn size(&self) -> Result<Size> {
        Ok(self.inner.size().into())
    }

    fn set_size(&mut self, _size: Size) -> Result<()> {
        Err(Error::NotSupported)
    }
}

mod std_impls {
    use super::{Backend, NotcursesBackend};
    use std::fmt;

    impl fmt::Debug for NotcursesBackend {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "NotcursesBackend {{ {}, {:?}, {:?} }}",
                self.version_string(),
                self.inner,
                self.root,
            )
        }
    }

    // impl From<Notcurses> for NotcursesBackend {
    //     fn from(nc: Notcurses) -> NotcursesBackend {
    //         let root: nc.cli_plane().unwrap() // can fail :S
    //         NotcursesBackend {
    //             inner: nc,
    //             root,
    //         }
    //     }
    // }
    // impl From<NotcursesBackend> for Notcurses {
    //     fn from(ui: NotcursesBackend) -> Notcurses {
    //         ui.inner
    //     }
    // }
}
