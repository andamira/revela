// revela::visual::text
//
//! Working with texts.
//

#[cfg(feature = "alloc")]
use alloc::string::String;

use crate::all::{Position, RevelaResult as Result, Visual};

/// A multi-layered grid of text.
pub trait TextGrid: Visual {
    /* cursor */

    /// Returns the cursor position.
    fn cursor(&self) -> Position;

    /// Moves the cursor to the home position `(0, 0)`.
    fn cursor_home(&mut self) {
        self.cursor_to(Position::new(0, 0)).unwrap()
    }

    /// Moves the cursor to the indicated `position`.
    fn cursor_to(&mut self, position: impl Into<Position>) -> Result<()>;

    /// Moves the cursor to the indicated `row`.
    fn cursor_to_row(&mut self, row: i32) -> Result<()>;

    /// Moves the cursor to the indicated `col`umn.
    fn cursor_to_col(&mut self, col: i32) -> Result<()>;

    /// Moves the cursor a relative `offset` Position.
    fn cursor_offset(&mut self, offset: impl Into<Position>) -> Result<()>;

    /* write */

    // https://docs.rs/notcurses/latest/notcurses/struct.Plane.html#text-and-cells

    /// Prints a `string` to the current cursor position.
    ///
    /// Returns the number of columns the cursor has advanced.
    fn print(&mut self, string: &str) -> Result<u32>;

    /// Erases the full contents.
    fn erase(&mut self);

    /// Returns a string with the full contents.
    #[cfg(feature = "alloc")]
    fn contents(&mut self) -> Result<String>;

    // IMPROVE: depends on layers
    fn raster(&mut self) -> Result<()>;
    fn render(&mut self) -> Result<()>;
    // fn render_raster(&mut self) -> Result<()>;
}
