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

    let mut notcurses_ui = NotcursesUi::new()?;
    let nui = &mut notcurses_ui;
    // nui.enable_mouse()?;

    /* */

    // TEMP
    let mut t0 = nui.new_root_child((1, 2, 12, 7))?;
    debug!["t0 putstr: {:?}", t0.putstr("hello world")?];

    /* */

    let mut l = Looper::new();
    assert![l.add_rate("input", Rate::with_tps(200.), true).is_ok()];
    assert![l.add_rate("render", Rate::with_tps(24.), true).is_ok()];
    l.reset();

    #[cfg(feature = "gilrs")]
    let mut gilrs = GilrsUi::new()?;

    #[cfg(feature = "midir")]
    let mut midir = {
        let mut midir = MidirUi::new()?;
        midir.in_connect_all()?;
        debug!["{midir:?}"];
        midir
    };

    loop {
        if let Some((now0, _delta0)) = l.measure() {
            /* all kinds of input */

            if let Some(_) = l.do_tick(now0, "input") {
                /* input: midi */

                #[cfg(feature = "midir")]
                input_midir(&mut midir)?;

                /* input: gamepad */

                #[cfg(feature = "gilrs")]
                input_gilrs(&mut gilrs)?;

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
                            Code::Up => t0.offset((0, -1))?,
                            Code::Down => t0.offset((0, 1))?,
                            Code::Left => t0.offset((-1, 0))?,
                            Code::Right => t0.offset((1, 0))?,
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

#[cfg(feature = "gilrs")]
fn input_gilrs(gilrs: &mut GilrsUi) -> UiResult<()> {
    let gilrs_event = gilrs.poll_event()?;
    match gilrs_event {
        Event::Gamepad(g) => {
            debug!["gilrs: {g:?}"];
            debug!["counter: {:?}", gilrs.counter()];
        }
        _ => (),
    }
    gilrs.increment();
    Ok(())
}

#[cfg(feature = "midir")]
fn input_midir(midir: &mut MidirUi) -> UiResult<()> {
    let midir_event = midir.poll_event()?;
    match midir_event {
        Event::Midi(m) => {
            trace!["midir: {m:?}"];
        }
        _ => (),
    }
    Ok(())
}
