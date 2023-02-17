// revela::core
//
//! Core traits and types.
//

pub mod events;
pub mod text;
mod ui;
mod window;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{events::all::*, text::TextGrid, ui::Ui, window::Window};
}
