// revela::core::text
//
//! Working with texts.
//

use crate::all::{Position, Size, UiResult as Result};

/// A grid of text.
pub trait TextGrid {
    /// Returns the size of the grid in cells.
    fn size(&self) -> Size;

    fn cursor(&self) -> Position;
    fn cursor_to(&mut self, position: impl Into<Position>) -> Result<()>;
    fn cursor_to_row(&mut self, row: i32) -> Result<()>;
    fn cursor_to_col(&mut self, col: i32) -> Result<()>;

    /// Writes a string to the current cursor position.
    ///
    /// Returns the number of columns the cursor has advanced.
    fn putstr(&mut self, string: &str) -> Result<u32>;
}
