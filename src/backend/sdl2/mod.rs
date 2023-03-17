// revela::backends::sdl2
//
//! `sdl2` backend.
//

/// Re-export of the [`sdl2`](https://docs.rs/sdl2) crate.
#[doc(inline)]
pub use ::sdl2;

mod backend;
mod canvas;
mod events;

pub use backend::Sdl2Backend;
pub use canvas::Sdl2Canvas;
pub use events::Sdl2EventSource;
