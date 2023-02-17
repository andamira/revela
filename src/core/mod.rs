// revela::core
//
//! Core traits and types.
//

mod backend;
pub mod events;
pub mod text;
mod window;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{backend::Backend, events::all::*, text::TextGrid, window::Window};
}
