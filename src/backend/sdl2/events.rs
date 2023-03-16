// revela::backend::sdl2::events
//
//!
//

use crate::all::{
    Event, EventKind, EventSource, KeyCode, KeyKind, KeyModifiers, MediaKey, ModifierKey,
    MouseEvent, MouseKind, Position, RevelaError as Error, RevelaResult as Result, Sdl2Backend,
    WindowEvent,
};
use sdl2::{
    event::{
        DisplayEvent as Sdl2DisplayEvent, Event as Sdl2Event, EventType as Sdl2EventType,
        WindowEvent as Sdl2WindowEvent,
    },
    keyboard::{Keycode as Sdl2Keycode, Scancode as Sdl2Scancode},
    mouse::{MouseButton as Sdl2MouseButton, MouseWheelDirection as Sdl2MouseWheelDirection},
    EventPump as Sdl2EventPump,
};

/// `sdl2` [`EventSource`].
pub struct Sdl2EventSource {
    // https://docs.rs/sdl2/latest/sdl2/struct.Sdl2EventPump.html
    pub(crate) pump: Sdl2EventPump,
}

///
impl Sdl2EventSource {
    /// Creates a new Sdl Ui.
    pub fn new(sdl: &Sdl2Backend) -> Result<Self> {
        Ok(Self {
            pump: sdl.sdl.event_pump()?,
        })
    }

    pub fn into_inner(self) -> Sdl2EventPump {
        self.pump
    }
}

mod std_impls {
    use super::{Sdl2EventPump, Sdl2EventSource};
    use std::fmt;

    impl fmt::Debug for Sdl2EventSource {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Sdl2EventSource {{ … }}")
        }
    }

    impl From<Sdl2EventPump> for Sdl2EventSource {
        fn from(pump: Sdl2EventPump) -> Sdl2EventSource {
            Sdl2EventSource { pump }
        }
    }
    impl From<Sdl2EventSource> for Sdl2EventPump {
        fn from(sdl: Sdl2EventSource) -> Sdl2EventPump {
            sdl.pump
        }
    }
}

impl EventSource for Sdl2EventSource {
    fn wait_event(&mut self) -> Result<Event> {
        todo![]
        // Ok(self.inner.get_event()?.into())
    }

    fn poll_event(&mut self) -> Result<Event> {
        todo![]
        // Ok(self.inner.poll_event()?.into())

        // old
        // Ok(BackendEvent::Sdl(self.pump.poll_event().ok_or_else(|| Error::new("sdl2 poll"))?))
    }
}
