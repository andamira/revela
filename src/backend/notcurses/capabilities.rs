// revela::backend::nc::capabilities
//
//!
//
// THINK

pub struct NotcursesCapabilities {}

// use crate::all::{Capabilities, PixelCapabilities};
// use notcurses::Capabilities as NcCaps;
//
// impl From<NcCaps> for Capabilities {
//     fn from(caps: NcCaps) -> Capabilities {
//         // TEMP
//         // let text = Some(TextCapabilities {
//         //     support: true,
//         //     variable: false
//         // });
//         //
//         // let pixel = Some(PixelCapabilities {
//         //     support: nc.pixel(), // blitter?
//         //     native: nc.pixel(),
//         // });
//
//         let multi_window = false;
//         let pixels = Some(PixelCapabilities { blit: caps.pixel() });
//         let cell_grid = true;
//         let utf8 = caps.utf8();
//         let sound = false;
//
//         Capabilities {
//             pixels,
//             cell_grid,
//             multi_window,
//             utf8,
//             sound,
//         }
//     }
// }
