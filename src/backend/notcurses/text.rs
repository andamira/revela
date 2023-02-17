// revela::backend::nc::text
//
//!
//

use super::NotcursesBackend;
use crate::all::{Clamper as C, Position, RevelaResult as Result, Size, TextGrid, Zone};
use ::notcurses::Plane;

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

    //

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

impl TextGrid for NotcursesTextGrid {
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
        Ok(self.inner.move_rel(position.into())?)
    }

    /* cursor */

    fn cursor(&self) -> Position {
        self.inner.cursor().into()
    }

    /// # Errors
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
    /// Errors if the coordinates exceed the inner plane’s dimensions,
    /// and the cursor will remain unchanged in that case.
    fn cursor_offset(&mut self, offset: impl Into<Position>) -> Result<()> {
        let (x, y): (i32, i32) = self.cursor().into();
        let (xo, yo): (i32, i32) = offset.into().into();
        Ok(self.inner.cursor_move_to((x + xo, y + yo))?)
    }

    /* */

    /// # Errors
    /// - if the position falls outside the plane’s area.
    /// - if a glyph can’t fit in the current line, unless scrolling is enabled.
    fn putstr(&mut self, string: &str) -> Result<u32> {
        Ok(self.inner.putstr(string)?)
    }

    fn raster(&mut self) -> Result<()> {
        Ok(self.inner.rasterize()?)
    }
    fn render(&mut self) -> Result<()> {
        Ok(self.inner.render()?)
    }
}
