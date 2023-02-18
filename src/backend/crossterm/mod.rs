// revela::backend::crossterm
//
//! `crossterm` backend.
//

/// Re-export of the [`crossterm`](https://docs.rs/crossterm) crate.
#[doc(inline)]
pub use ::crossterm;

mod backend;
mod events;

pub use backend::CrosstermBackend;
