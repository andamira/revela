// revela::core
//
//! Core traits and types.
//

pub mod events;
mod ui;
mod window;

#[doc(inline)]
pub use events::{
    keyboard::{Code, KeyEvent, KeyModifiers, MediaKey, ModifierKey},
    // gamepad::{},
    // midi::{},
    // mouse::{},
    window::WindowEvent,
    Event,
    EventSource,
};
pub use ui::Ui;
pub use window::Window;
