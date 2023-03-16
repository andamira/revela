// revela::backends::sdl2
//
//! `sdl2` backend.
//

/// Re-export of the [`sdl2`](https://docs.rs/sdl2) crate.
#[doc(inline)]
pub use ::sdl2;

// mod canvas; // TODO
mod backend;
mod events;

// pub use canvas::Sdl2Canvas;
pub use backend::Sdl2Backend;
pub use events::Sdl2EventSource;
