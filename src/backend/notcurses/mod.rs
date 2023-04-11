// revela::backend::notcurses
//
//! `notcurses` backend.
//

/// Re-export of the [`notcurses`](https://docs.rs/notcurses) crate.
#[doc(inline)]
pub use ::notcurses;

mod backend;
mod canvas;
// mod capabilities;
mod events;
mod text_grid;

pub use backend::NotcursesBackend;
pub use canvas::NotcursesCanvas;
pub use text_grid::NotcursesTextGrid;
