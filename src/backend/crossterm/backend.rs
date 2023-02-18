// revela::backend::crossterm::backend
//
//! Backend wrapper for `crossterm`
//
// ISSUES
// - signals
//   - add Event::Terminate: https://github.com/crossterm-rs/crossterm/issues/554
//   - support ctrl+z + fg: https://github.com/crossterm-rs/crossterm/issues/494
//
// TODO
// - window refresh, render

use ::crossterm::{event, execute, terminal};

use core::time::Duration;
use std::io;

use crate::all::{Backend, Event, EventSource, RevelaResult as Result, Size, Window};

/// `crossterm` interface.
///
/// It implements the following traits: [`Backend`], [`Window`], [`EventSource`].
//
// https://docs.rs/crossterm/latest/crossterm/terminal/index.html
pub struct CrosstermBackend {
    // raw_mode: bool,
}
// impl Drop for CrosstermBackend {
//     fn drop(&mut self)  {
//         if self.raw_mode {
//             self.set_raw_mode(false);
//         }
//     }
// }

impl CrosstermBackend {
    /// Creates a new `CrosstermBackend`.
    pub fn new() -> Result<Self> {
        // let mut inner = Notcurses::new()?;
        // let root = NotcursesTextGrid::from_plane(inner.cli_plane()?);
        Ok(Self { /* raw_mode: false */ })
    }

    /// Tells whether the raw mode is enabled.
    #[inline]
    pub fn is_raw_mode(&self) -> Result<bool> {
        Ok(terminal::is_raw_mode_enabled()?)
    }

    /// Enables the raw mode.
    #[inline]
    pub fn enable_raw_mode(&self) -> Result<()> {
        Ok(terminal::enable_raw_mode()?)
    }

    /// Disables the raw mode.
    #[inline]
    pub fn disable_raw_mode(&self) -> Result<()> {
        Ok(terminal::disable_raw_mode()?)
    }

    /// Switches to the alternate screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html
    pub fn enter_alternate_screen(&self) -> Result<()> {
        Ok(execute!(io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Switches back to the main screen.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html
    pub fn leave_alternate_screen(&self) -> Result<()> {
        Ok(execute!(io::stdout(), terminal::EnterAlternateScreen)?)
    }

    /// Enables bracketed paste mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableBracketedPaste.html
    pub fn enable_bracketed_paste(&self) -> Result<()> {
        Ok(execute!(io::stdout(), event::EnableBracketedPaste)?)
    }

    /// Disables bracketed paste mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableBracketedPaste.html
    pub fn disable_bracketed_paste(&self) -> Result<()> {
        Ok(execute!(io::stdout(), event::DisableBracketedPaste)?)
    }

    // /// Returns a shared reference to the root text grid of the window.
    // pub fn ref_root(&self) -> &NotcursesTextGrid {
    //     &self.root
    // }
    //
    // /// Returns an exclusive reference to the root text grid of the window.
    // pub fn mut_root(&mut self) -> &mut NotcursesTextGrid {
    //     &mut self.root
    // }
    //
    // pub fn new_root_child(&mut self, zone: impl Into<Zone>) -> Result<NotcursesTextGrid> {
    //     self.root.new_child(zone)
    // }
    //
    // //
    //
    // /// Creates a new `NotcursesBackend` from an existing `notcurses` instance,
    // /// and an optional `root` [`Plane`].
    // ///
    // /// When no plane is given, it will try to instantiate the *cli* plane.
    // ///
    // /// # Errors
    // /// Returns an error if the CLI plane has been already instantiated.
    // pub fn from_notcurses(nc: Notcurses, root: Option<Plane>) -> Result<Self> {
    //     let mut inner = nc;
    //     let root = NotcursesTextGrid::from_plane(
    //         // inner.cli_plane()?
    //         if let Some(plane) = root {
    //             plane
    //         } else {
    //             inner.cli_plane()?
    //         },
    //     );
    //     Ok(Self { inner, root })
    // }
    //
    // pub fn into_inner(self) -> Notcurses {
    //     self.inner
    // }
    // pub fn ref_inner(&self) -> &Notcurses {
    //     &self.inner
    // }
    // pub fn mut_inner(&mut self) -> &mut Notcurses {
    //     &mut self.inner
    // }
}

impl CrosstermBackend {
    // /// Enables receiving mouse events.
    // pub fn enable_mouse(&mut self) -> Result<()> {
    //     Ok(self.inner.mice_enable(::notcurses::MiceEvents::All)?)
    // }
    // /// Disables receiving mouse events.
    // pub fn disable_mouse(&mut self) -> Result<()> {
    //     Ok(self.inner.mice_disable()?)
    // }
}

impl Backend for CrosstermBackend {
    // - supports_keyboard_enhancement
    // fn capabilities(&self) -> Capabilities {
    //     self.inner.capabilities().into()
    // }

    fn version(&self) -> (u32, u32, u32) {
        (0, 2, 6)
    }
}

impl EventSource for CrosstermBackend {
    // https://docs.rs/crossterm/latest/crossterm/event/fn.read.html
    fn wait_event(&mut self) -> Result<Event> {
        Ok(event::read()?.into())
    }

    // https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
    fn poll_event(&mut self) -> Result<Event> {
        if event::poll(Duration::from_millis(0))? {
            Ok(event::read()?.into())
        } else {
            Ok(Event::None)
        }
    }
}

impl Window for CrosstermBackend {
    fn refresh(&mut self) -> Result<()> {
        todo![]
        // let _ = self.inner.refresh()?;
        // Ok(())
    }
    fn render(&mut self) -> Result<()> {
        todo![]
        // self.root.render()?;
        // Ok(())
    }

    #[inline]
    fn size(&self) -> Result<Size> {
        Ok(terminal::size()?.into())
    }

    // https://docs.rs/crossterm/latest/crossterm/terminal/struct.SetSize.html
    //
    // ISSUES:
    // - SetSize does not resize xterm: https://github.com/crossterm-rs/crossterm/issues/715
    // - Setsize doesnt work: https://github.com/crossterm-rs/crossterm/issues/678
    #[inline]
    fn set_size(&mut self, size: Size) -> Result<()> {
        let (cols, rows): (u16, u16) = size.into();
        Ok(execute!(io::stdout(), terminal::SetSize(cols, rows))?)
    }
}

mod std_impls {
    use super::{Backend, CrosstermBackend};
    use std::fmt;

    impl fmt::Debug for CrosstermBackend {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "CrosstermBackend {{ {} }}",
                self.version_string(),
                // self.…,
            )
        }
    }
}
