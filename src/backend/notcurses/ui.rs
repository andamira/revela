// revela::backend::nc::ui
//
//! Ui wrapper for notcurses
//

use notcurses::{Notcurses, Plane};

use crate::all::{Code, Event, EventSource, Size, Ui, UiResult, Window};
// Capabilities, Unit

/// `notcurses` user interface.
///
/// It implements the following traits: [`Ui`], [`Window`], [`EventSource`].
pub struct NotcursesUi {
    inner: Notcurses,
    /// the root *cli* plane
    pub(crate) root: Plane, // MAYBE remove the pub(crate)
}

impl NotcursesUi {
    /// Creates a new `NotcursesUi`.
    pub fn new() -> UiResult<Self> {
        let mut inner = Notcurses::new()?;
        let root = inner.cli_plane()?;
        Ok(Self { inner, root })
    }

    //

    /// Creates a new `NotcursesUi` from an existing notcurses instance,
    /// and an optional `root` [`Plane`].
    ///
    /// When no plane is given, it will try to instantiate the *cli* plane.
    ///
    /// # Errors
    /// Returns an error if the CLI plane has been already instantiated.
    pub fn from_notcurses(nc: Notcurses, root: Option<Plane>) -> UiResult<Self> {
        let mut inner = nc;
        let root = if let Some(plane) = root {
            plane
        } else {
            inner.cli_plane()?
        };
        Ok(Self { inner, root })
    }

    //

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

impl NotcursesUi {
    /// Enables receiving mouse events.
    pub fn enable_mouse(&mut self) -> UiResult<()> {
        Ok(self.inner.mice_enable(::notcurses::MiceEvents::All)?)
    }
    /// Disables receiving mouse events.
    pub fn disable_mouse(&mut self) -> UiResult<()> {
        Ok(self.inner.mice_disable()?)
    }
}

impl Ui for NotcursesUi {
    // impl Capabilities?
    // fn capabilities(&self) -> Capabilities {
    //     self.inner.capabilities().into()
    // }
    fn version(&self) -> (u32, u32, u32) {
        let v = Notcurses::version_components();
        (v.0, v.1, v.2)
    }

    // fn try_into_backend(self, backend: Backend) -> Option<BackendElement> {
    //     match backend {
    //         Backend::Notcurses => Some(BackendElement::NotcursesUi(self)),
    //         #[allow(unreachable_patterns)] // TEMP
    //         _ => None,
    //     }
    // }
}

impl EventSource for NotcursesUi {
    fn wait_event(&mut self) -> UiResult<Event> {
        Ok(self.inner.get_event()?.into())
    }

    fn poll_event(&mut self) -> UiResult<Event> {
        Ok(self.inner.poll_event()?.into())
    }
}

impl Window for NotcursesUi {
    fn refresh(&mut self) -> UiResult<()> {
        let _ = self.inner.refresh()?;
        Ok(())
    }
    fn render(&mut self) -> UiResult<()> {
        self.root.render()?;
        Ok(())
    }

    fn size(&self) -> Size {
        self.inner.size().into()
    }
}

mod std_impls {
    use super::{NotcursesUi, Ui};
    use std::fmt;

    impl fmt::Debug for NotcursesUi {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "NotcursesUi {{ {}, {:?}, {:?} }}",
                self.version_string(),
                self.inner,
                self.root,
            )
        }
    }

    // impl From<Notcurses> for NotcursesUi {
    //     fn from(nc: Notcurses) -> NotcursesUi {
    //         let root: nc.cli_plane().unwrap() // can fail :S
    //         NotcursesUi {
    //             inner: nc,
    //             root,
    //         }
    //     }
    // }
    // impl From<NotcursesUi> for Notcurses {
    //     fn from(ui: NotcursesUi) -> Notcurses {
    //         ui.inner
    //     }
    // }
}
