// revela::visual::text
//
//! Working with texts.
//

use crate::all::{Position, RevelaResult as Result, Visual};

/// A multi-layered grid of text.
pub trait TextGrid: Visual {
    /// Returns the cursor position.
    fn cursor(&self) -> Position;
    /// Moves the cursor to the indicated `position`.
    fn cursor_to(&mut self, position: impl Into<Position>) -> Result<()>;
    /// Moves the cursor to the indicated `row`.
    fn cursor_to_row(&mut self, row: i32) -> Result<()>;
    /// Moves the cursor to the indicated `col`umn.
    fn cursor_to_col(&mut self, col: i32) -> Result<()>;
    /// Moves the cursor a relative `offset` Position.
    fn cursor_offset(&mut self, offset: impl Into<Position>) -> Result<()>;

    /// Writes a string to the current cursor position.
    /// Returns the number of columns the cursor has advanced.
    fn putstr(&mut self, string: &str) -> Result<u32>;

    // IMPROVE: depends on layers
    fn raster(&mut self) -> Result<()>;
    fn render(&mut self) -> Result<()>;
    // fn render_raster(&mut self) -> Result<()>;
}
