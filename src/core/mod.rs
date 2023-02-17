// revela::core
//
//! Core traits and types.
//

pub mod text;
mod ui;
mod window;

pub use text::TextGrid;
pub use ui::Ui;
pub use window::Window;

pub mod events;
#[doc(inline)]
pub use events::{
    gamepad::{GamepadAxis, GamepadButton, GamepadEvent},
    keyboard::{Code, KeyEvent, KeyModifiers, MediaKey, ModifierKey},
    window::WindowEvent,
    Event, EventSource,
};
