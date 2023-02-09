// revela::layout::position
//
///
//
use super::Distance as D;

/* definitions */

/// A 2D position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    /// Defines a new `Position` with the given coordinates.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x: D::clamp(x),
            y: D::clamp(y),
        }
    }

    /// Get x.
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }
    /// Get y.
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Set x.
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.x = D::clamp(x);
    }
    /// Set y.
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.y = D::clamp(y);
    }
}

mod conversions {
    use super::{Position, D};

    /// # convert
    impl Position {
        pub const fn as_tuple(&self) -> (i32, i32) {
            (self.x, self.y)
        }
        pub const fn from_tuple(tup: (i32, i32)) -> Position {
            Self::new(D::clamp(tup.0), D::clamp(tup.1))
        }

        pub const fn as_tuple_u32(&self) -> (u32, u32) {
            (D::clamp_to_u32(self.x), D::clamp_to_u32(self.y))
        }
        pub const fn from_tuple_u32(tup: (u32, u32)) -> Position {
            Self::new(D::clamp_from_u32(tup.0), D::clamp_from_u32(tup.1))
        }

        pub const fn as_tuple_i16(&self) -> (i16, i16) {
            (D::clamp_to_i16(self.x), D::clamp_to_i16(self.y))
        }
        pub const fn from_tuple_i16(tup: (i16, i16)) -> Position {
            Self::new(D::clamp_from_i16(tup.0), D::clamp_from_i16(tup.1))
        }

        pub fn as_tuple_usize(&self) -> (usize, usize) {
            (
                D::clamp_positive_to_usize(self.x),
                D::clamp_positive_to_usize(self.y),
            )
        }
        pub fn from_tuple_usize(tup: (usize, usize)) -> Position {
            Self::new(
                D::clamp_positive_from_usize(tup.0),
                D::clamp_positive_from_usize(tup.1),
            )
        }
    }

    impl From<(i32, i32)> for Position {
        fn from(tup: (i32, i32)) -> Position {
            Self::from_tuple(tup)
        }
    }
    impl From<Position> for (i32, i32) {
        fn from(p: Position) -> (i32, i32) {
            p.as_tuple()
        }
    }

    impl From<(i16, i16)> for Position {
        fn from(tup: (i16, i16)) -> Position {
            Self::from_tuple_i16(tup)
        }
    }
    impl From<Position> for (i16, i16) {
        fn from(s: Position) -> (i16, i16) {
            s.as_tuple_i16()
        }
    }

    impl From<(u32, u32)> for Position {
        fn from(tup: (u32, u32)) -> Position {
            Self::from_tuple_u32(tup)
        }
    }
    impl From<Position> for (u32, u32) {
        fn from(s: Position) -> (u32, u32) {
            s.as_tuple_u32()
        }
    }
}
