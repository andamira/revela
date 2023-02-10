// revela example ui-notcurses
//
//!
//

use depura::*;
use espera::*;
use revela::all::*;

fn main() -> UiResult<()> {
    Logger::new("revela example ui-notcurses")
        .file("log-ui-notcurses.log")
        .target_level_all()
        .init()
        .unwrap();
    info!["press Escape to exit"];

    let mut nui = NotcursesUi::new()?;
    // nui.enable_mouse()?;

    loop {
        /* input */

        let event = nui.wait_event()?;
        // let event = nui.poll_event()?;

        match event {
            Event::Window(w) => {
                debug!["window: {w:?}"];
                match w {
                    // FIXME: https://github.com/dankamongmen/notcurses/issues/2702
                    WindowEvent::Continue => {
                        debug!["Issue continuing from resize"];
                    }
                    WindowEvent::Resized => {
                        // Key::Resize => nui.refresh()?, // already does on render
                    }
                    _ => (),
                }
            }
            Event::Key(k) => {
                debug!["key: {k:?}"];
                match k.code {
                    Code::Escape | Code::Char('q') => break,
                    _ => (),
                }
            }
            // Event::Mouse(m) => {
            //     debug!["mouse: {m:?}"];
            // }
            _ => (),
        }

        /* render */

        nui.render()?;

        // TEMP print dimensions
        // debug!["{:?}", nui.size()];

        // info!["loop"];
        sleep4![0, 40];
    }

    info!["bye!"];
    Ok(())
}
