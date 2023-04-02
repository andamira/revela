// revela::visual
//
//!
//

use crate::error::RevelaResult as Result;

mod canvas;
mod layout;
mod text_grid;
mod window;

/// Re-export of the [`acolor`](https://docs.rs/acolor) crate.
#[doc(inline)]
pub use ::acolor;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        acolor::{
            Color, LinearSrgb32, LinearSrgba32, Oklab32, Oklch32, Srgb32, Srgb8, Srgba32, Srgba8,
        },
        canvas::Canvas,
        layout::{Clamper, Position, Size, Zone},
        text_grid::TextGrid,
        window::Window,
        Visual,
    };
}

/// Common visual trait.
pub trait Visual {

    fn zone(&self) -> Zone;
    fn size(&self) -> Size {
        self.zone().size()
    }
    fn position(&self) -> Position {
        self.zone().position()
    }

    /// Moves the visual an `offset` relative to its current position.
    fn offset(&mut self, offset: impl Into<Position>) -> Result<()>;

    /// Moves the visual to a new `position`, relative to its parent.
    fn move_to(&mut self, position: impl Into<Position>) -> Result<()>;
}
