// revela::canvas
//
//!
//

use acolor::Color;

use crate::all::{Position, RevelaResult as Result};

/// Canvas trait.
pub trait Canvas {
    ///
    type Color: Color;

    ///
    fn get_pixel(&self, p: Position) -> Result<Self::Color>;

    ///
    fn set_pixel(&mut self, p: Position, color: Self::Color) -> Result<()>;
}
