// revela::example::ui-notcurses
//
//! # Notcurses UI example
//!
//! Optionally supports `gilrs` backend.
//

use depura::*;
use repite::*;
use revela::all::*;

fn main() -> UiResult<()> {
    Logger::new("revela example ui-notcurses")
        .file("log-ui-notcurses.log")
        .target_level_all()
        //
        .ignore("gilrs::ff")
        .ignore("gilrs_core")
        //
        .init()
        .unwrap();
    info!["press Escape to exit"];

    let mut nui = NotcursesUi::new()?;
    // nui.enable_mouse()?;

    let mut l = Looper::new();
    assert![l.add_rate("input", Rate::with_tps(200.), true).is_ok()];
    assert![l.add_rate("render", Rate::with_tps(24.), true).is_ok()];
    l.reset();

    #[cfg(feature = "gilrs")]
    let mut gilrs = GilrsUi::new()?;

    loop {
        if let Some((now0, _delta0)) = l.measure() {
            /* input */

            if let Some(_) = l.do_tick(now0, "input") {
                /* input: gamepad */

                #[cfg(feature = "gilrs")]
                {
                    let gilrs_event = gilrs.poll_event()?;
                    match gilrs_event {
                        Event::Gamepad(g) => {
                            debug!["gilrs: {g:?}"];
                            debug!["counter: {:?}", gilrs.counter()];
                        }
                        _ => (),
                    }
                    gilrs.increment();
                }

                /* input: notcurses */

                let nui_event = nui.poll_event()?;
                match nui_event {
                    Event::Window(w) => {
                        debug!["window: {w:?}"];
                        match w {
                            // FIX: lost raw mode
                            // https://github.com/dankamongmen/notcurses/issues/2702
                            WindowEvent::Continue => {
                                debug!["welcome back"];
                            }
                            WindowEvent::Resized => {
                                debug!["{:?}", nui.size()];
                            }
                            _ => (),
                        }
                    }
                    Event::Key(k) => {
                        debug!["key: {k:?}"];
                        match k.code {
                            Code::Escape | Code::Char('q') => break,
                            Code::Char('l') => l.log_all_rates(),
                            _ => (),
                        }
                    }
                    // Event::Mouse(m) => {
                    //     debug!["mouse: {m:?}"];
                    // }
                    _ => (),
                }
            }

            /* render */

            if let Some(_) = l.do_tick(now0, "render") {
                nui.render()?;
            }
        }
        l.sleep(Duration::microseconds(1));
    }

    info!["bye!"];
    Ok(())
}
