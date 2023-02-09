// revela::layout::zone
//
//!
//

use super::{Position, Size};

/// A 2D zone combines a [`Position`] and a [`Size`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Zone {
    pub p: Position,
    pub s: Size,
}

/// # accesors
impl Zone {
    pub fn new(position: Position, size: Size) -> Self {
        Zone {
            p: position,
            s: size,
        }
    }

    pub fn new_raw(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new(Position::new(x, y), Size::new(width, height))
    }

    /// Get the position.
    pub const fn position(&self) -> Position {
        self.p
    }
    /// Mutate the position.
    pub fn mut_position(&mut self, position: Position) {
        self.p = position;
    }
    /// Chain-set the position.
    pub fn set_position(mut self, position: impl Into<Option<Position>>) -> Self {
        if let Some(p) = position.into() {
            self.p = p
        }
        self
    }

    /// Get the size.
    pub const fn size(&self) -> Size {
        self.s
    }
    /// Mutate the size.
    pub fn mut_size(mut self, size: Size) {
        self.s = size;
    }
    /// Chain-set the size.
    pub fn set_size(mut self, size: impl Into<Option<Size>>) -> Self {
        if let Some(s) = size.into() {
            self.s = s;
        }
        self
    }

    /// Get the `x` position.
    pub const fn x(&self) -> i32 {
        self.p.x()
    }
    /// Set the `x` position.
    pub fn set_x(&mut self, x: i32) {
        self.p.set_x(x)
    }

    /// Get the `y` position.
    pub const fn y(&self) -> i32 {
        self.p.y()
    }
    /// Set the `y` position.
    pub fn set_y(&mut self, y: i32) {
        self.p.set_y(y)
    }

    /// Get the `width`.
    pub const fn w(&self) -> i32 {
        self.s.w()
    }
    /// Set the `width`.
    pub fn set_w(&mut self, width: i32) {
        self.s.set_w(width)
    }

    /// Get the `height`.
    pub const fn h(&self) -> i32 {
        self.s.h()
    }
    /// Set the `height`.
    pub fn set_h(&mut self, height: i32) {
        self.s.set_h(height)
    }
}
