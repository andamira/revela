// revela::backend::notcurses::text
//
//!
//

use super::NotcursesBackend;
use crate::all::{Clamper as C, Position, RevelaResult as Result, Size, TextGrid, Zone, Visual};
use ::notcurses::Plane;

/// `notcurses` [`TextGrid`] layer.
#[derive(Debug)]
pub struct NotcursesTextGrid {
    inner: Plane,
    // z-index
}

impl NotcursesTextGrid {
    /// Creates a new standalone text grid.
    pub fn new(nc: &mut NotcursesBackend, zone: impl Into<Zone>) -> Result<Self> {
        let zone = zone.into();
        Ok(Self {
            inner: Plane::new_sized_at(nc.mut_inner(), zone.s, zone.p)?,
        })
    }

    /// Creates a new text grid that has the current text grid as a parent.
    pub fn new_child(&mut self, zone: impl Into<Zone>) -> Result<Self> {
        let zone = zone.into();
        Ok(Self {
            inner: Plane::new_child_sized_at(self.mut_inner(), zone.s, zone.p)?,
        })
    }

    /* scroll */

    /// Returns `true` if the text grid is set to scroll.
    pub fn is_scrolling(&self) -> bool {
        self.inner.is_scrolling()
    }

    /// Sets the text grid to `scroll`, or not.
    #[inline]
    pub fn set_scrolling(&mut self, scroll: bool) -> bool {
        self.inner.set_scrolling(scroll)
    }

    //

}

impl NotcursesTextGrid {
    pub fn from_plane(plane: Plane) -> Self {
        Self { inner: plane }
    }
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

impl Visual for NotcursesTextGrid {
    fn zone(&self) -> Zone {
        Zone::new(self.position(), self.size())
    }
    fn size(&self) -> Size {
        self.inner.size().into()
    }
    fn position(&self) -> Position {
        self.inner.position().into()
    }

    fn offset(&mut self, offset: impl Into<Position>) -> Result<()> {
        Ok(self.inner.move_rel(offset.into())?)
    }
    fn move_to(&mut self, position: impl Into<Position>) -> Result<()> {
        Ok(self.inner.move_to(position.into())?)
    }

}

impl TextGrid for NotcursesTextGrid {
    /* cursor */

    /// Returns the current cursor position.
    #[inline(always)]
    fn cursor(&self) -> Position {
        self.inner.cursor().into()
    }

    /// Moves the cursor to the home position (`0, 0`).
    #[inline(always)]
    fn cursor_home(&mut self) {
        self.inner.cursor_home()
    }

    /// Moves the cursor to the specified position.
    ///
    /// # Errors
    /// If the coordinates exceed the inner plane’s dimensions.
    /// The cursor will remain unchanged in that case.
    #[inline]
    fn cursor_to(&mut self, position: impl Into<Position>) -> Result<()> {
        Ok(self.inner.cursor_move_to(position.into())?)
    }

    /// Moves the cursor to the specified `row`.
    ///
    /// # Errors
    /// If the row number exceed the inner plane’s rows.
    /// The cursor will remain unchanged in that case.
    #[inline]
    fn cursor_to_row(&mut self, row: i32) -> Result<()> {
        Ok(self.inner.cursor_move_to_row(C::clamp_to_u32(row))?)
    }

    /// Moves the cursor to the specified `col`umn.
    ///
    /// # Errors
    /// If the row number exceed the inner plane’s columns.
    /// The cursor will remain unchanged in that case.
    #[inline]
    fn cursor_to_col(&mut self, col: i32) -> Result<()> {
        Ok(self.inner.cursor_move_to_col(C::clamp_to_u32(col))?)
    }

    /// Moves the cursor
    /// # Errors
    /// If the coordinates exceed the inner plane’s dimensions.
    /// The cursor will remain unchanged in that case.
    #[inline]
    fn cursor_offset(&mut self, offset: impl Into<Position>) -> Result<()> {
        let (x, y): (i32, i32) = self.cursor().into();
        let (xo, yo): (i32, i32) = offset.into().into();
        Ok(self.inner.cursor_move_to((x + xo, y + yo))?)
    }

    /* */

    /// # Errors
    /// - if the position falls outside the plane’s area.
    /// - if a glyph can’t fit in the current line, unless scrolling is enabled.
    fn print(&mut self, string: &str) -> Result<u32> {
        Ok(self.inner.putstr(string)?)
    }

    fn raster(&mut self) -> Result<()> {
        Ok(self.inner.rasterize()?)
    }
    fn render(&mut self) -> Result<()> {
        Ok(self.inner.render()?)
    }
}
