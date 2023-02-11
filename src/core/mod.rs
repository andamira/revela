// revela::core
//
//! Core traits and types.
//

pub mod events;
mod ui;
mod window;

#[doc(inline)]
pub use events::{
    gamepad::{GamepadAxis, GamepadButton, GamepadEvent, GamepadEventKind},
    keyboard::{Code, KeyEvent, KeyModifiers, MediaKey, ModifierKey},
    window::WindowEvent,
    Event, EventSource,
};
pub use ui::Ui;
pub use window::Window;
