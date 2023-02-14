// revela::backend::nc::text
//
//!
//

use super::NotcursesUi;
use crate::all::{Clamper as C, Position, Size, TextGrid, UiResult as Result, Zone};
use ::notcurses::Plane;

pub struct NotcursesTextGrid {
    inner: Plane,
    // z-index
}

impl NotcursesTextGrid {
    ///
    pub fn new(ui: &mut NotcursesUi, zone: impl Into<Zone>) -> Result<Self> {
        let zone = zone.into();
        Ok(Self {
            inner: Plane::new_sized_at(ui.mut_inner(), zone.s, zone.p)?,
        })
    }

    //

    pub fn into_inner(self) -> Plane {
        self.inner
    }
    pub fn ref_inner(&self) -> &Plane {
        &self.inner
    }
    pub fn mut_inner(&mut self) -> &mut Plane {
        &mut self.inner
    }
}

impl TextGrid for NotcursesTextGrid {
    fn size(&self) -> Size {
        self.inner.size().into()
    }

    fn cursor(&self) -> Position {
        self.inner.cursor().into()
    }

    /// Errors if the coordinates exceed the inner plane’s dimensions,
    /// and the cursor will remain unchanged in that case.
    fn cursor_to(&mut self, position: impl Into<Position>) -> Result<()> {
        Ok(self.inner.cursor_move_to(position.into())?)
    }

    /// # Errors
    /// Errors if the row number exceed the inner plane’s rows,
    /// and the cursor will remain unchanged in that case.
    fn cursor_to_row(&mut self, row: i32) -> Result<()> {
        Ok(self.inner.cursor_move_to_row(C::clamp_to_u32(row))?)
    }

    /// # Errors
    /// Errors if the row number exceed the inner plane’s columns,
    /// and the cursor will remain unchanged in that case.
    fn cursor_to_col(&mut self, col: i32) -> Result<()> {
        Ok(self.inner.cursor_move_to_col(C::clamp_to_u32(col))?)
    }

    /// # Errors
    /// - if the position falls outside the plane’s area.
    /// - if a glyph can’t fit in the current line, unless scrolling is enabled.
    fn putstr(&mut self, string: &str) -> Result<u32> {
        Ok(self.inner.putstr(string)?)
    }
}
