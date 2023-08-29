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

use crate::all::{
    Backend, Capabilities, ColorCapabilities, Event, EventSource, InputCapabilities,
    RevelaResult as Result, Size, TextGridCapabilities, Window, WindowCapabilities,
};

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
        Ok(Self { /* raw_mode: false */ })
    }

    /// Tells whether the raw mode is enabled.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.is_raw_mode_enabled.html
    #[inline]
    pub fn is_raw_mode(&self) -> Result<bool> {
        Ok(terminal::is_raw_mode_enabled()?)
    }

    /// Enables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.enable_raw_mode.html
    #[inline]
    pub fn enable_raw_mode(&self) -> Result<()> {
        Ok(terminal::enable_raw_mode()?)
    }

    /// Disables the raw mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/terminal/fn.disable_raw_mode.html
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

    /// Enables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableMouseCapture.html
    pub fn enable_mouse(&mut self) -> Result<()> {
        Ok(execute!(io::stdout(), event::EnableMouseCapture)?)
    }
    /// Disables receiving mouse events.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableMouseCapture.html
    pub fn disable_mouse(&mut self) -> Result<()> {
        Ok(execute!(io::stdout(), event::DisableMouseCapture)?)
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

    /// Enables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.EnableFocusChange.html
    pub fn enable_focus_change(&self) -> Result<()> {
        Ok(execute!(io::stdout(), event::EnableFocusChange)?)
    }

    /// Disables focus change mode.
    //
    // https://docs.rs/crossterm/latest/crossterm/event/struct.DisableFocusChange.html
    pub fn disable_focus_change(&self) -> Result<()> {
        Ok(execute!(io::stdout(), event::DisableFocusChange)?)
    }
}

impl Backend for CrosstermBackend {
    fn capabilities(&self) -> Capabilities {
        let color = Some(ColorCapabilities {
            rgb: true,
            palette_change: false,
            palette_size: ::crossterm::style::available_color_count(),
            ..Default::default()
        });

        let input = Some(InputCapabilities {
            keyboard: true,
            mouse: true,
            ..Default::default()
        });

        let text_grid = Some(TextGridCapabilities {
            // we don't unknown
            cell_size: None,
            // https://github.com/crossterm-rs/crossterm/issues/166
            custom_cell_size: false,
            // https://github.com/crossterm-rs/crossterm/issues/677
            unicode: true,
            // ..Default::default()
        });

        let window = Some(WindowCapabilities { multi: false });

        Capabilities {
            color,
            input,
            text_grid,
            window,
            ..Default::default()
        }
    }

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
