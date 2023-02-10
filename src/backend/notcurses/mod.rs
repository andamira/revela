// uiuiui::backend::nc
//
//! `notcurses` backend.
//

/// Re-export of the [`notcurses`](https://docs.rs/notcurses) crate.
#[doc(inline)]
pub use ::notcurses;

// mod capabilities;
mod events;
mod ui;
// mod widget;

pub use ui::NotcursesUi;
// pub use widget::{NotcursesText, NotcursesWidget, NotcursesWidgets};

/// General conversions
mod conversions {
    use crate::all::{Position, Size};
    use ::notcurses::{Position as NPosition, Size as NSize};

    impl From<NPosition> for Position {
        fn from(nc: NPosition) -> Position {
            nc.into_tuple().into()
        }
    }
    impl From<Position> for NPosition {
        fn from(pos: Position) -> NPosition {
            pos.as_tuple().into()
        }
    }

    impl From<NSize> for Size {
        fn from(nc: NSize) -> Size {
            nc.into_tuple().into()
        }
    }
    impl From<Size> for NSize {
        fn from(nc: Size) -> NSize {
            nc.as_tuple().into()
        }
    }
}
