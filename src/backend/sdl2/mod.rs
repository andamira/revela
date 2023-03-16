// revela::backends::sdl2
//
//! `sdl2` backend.
//

// mod canvas; // TODO
mod backend;
mod events;

// pub use canvas::Sdl2Canvas;
pub use backend::Sdl2Backend;
pub use events::Sdl2EventSource;
