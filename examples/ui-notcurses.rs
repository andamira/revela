// revela::example::ui-notcurses
//
//! # Notcurses UI example
//!
//! Optionally supported backends: `gilrs`, `midir`.
//

use depura::*;
use repite::*;
use revela::all::{RevelaResult as Result, *};

fn main() -> Result<()> {
    Logger::new("revela example ui-notcurses")
        .file("log-uis.log")
        .target_level_all()
        //
        .ignore("gilrs::ff")
        .ignore("gilrs_core")
        //
        .init()
        .unwrap();
    info!["press 'q' to exit"];

    let mut notcurses_ui = NotcursesBackend::new()?;
    let nc = &mut notcurses_ui;
    // nc.enable_mouse()?;

    /* */

    // TEMP
    let mut t0 = nc.new_root_child((1, 2, 12, 7))?;
    debug!["t0 putstr: {:?}", t0.putstr("hello world")?];

    /* */

    let mut l = Looper::new();
    assert![l.add_rate("input", Rate::with_tps(200.), true).is_ok()];
    assert![l.add_rate("render", Rate::with_tps(24.), true).is_ok()];
    l.reset();

    #[cfg(feature = "gilrs")]
    let mut gilrs = GilrsBackend::new()?;

    #[cfg(feature = "midir")]
    let mut midir = {
        let mut midir = MidirBackend::new()?;
        midir.in_connect_all()?;
        debug!["{midir:?}"];
        midir
    };

    loop {
        if let Some((now0, _delta0)) = l.measure() {
            /* inputs */

            if let Some(_) = l.do_tick(now0, "input") {
                #[cfg(feature = "midir")]
                input_midir(&mut midir)?;

                #[cfg(feature = "gilrs")]
                input_gilrs(&mut gilrs)?;

                // #[cfg(feature = "notcurses")]
                // input_notcurses(&mut nc)?;
                let nc_event = nc.poll_event()?;
                match nc_event.kind {
                    EventKind::Window(w) => {
                        debug!["window: {w:?}"];
                        match w {
                            // FIX: lost raw mode
                            // https://github.com/dankamongmen/notcurses/issues/2702
                            WindowEvent::Continue => {
                                debug!["welcome back"];
                            }
                            WindowEvent::Resized(_) => {
                                debug!["{:?}", nc.size()];
                            }
                            _ => (),
                        }
                    }
                    EventKind::Key(key) => {
                        debug!["key: {key:?}"];
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('l') => l.log_all_rates(),
                            KeyCode::Up => t0.offset((0, -1))?,
                            KeyCode::Down => t0.offset((0, 1))?,
                            KeyCode::Left => t0.offset((-1, 0))?,
                            KeyCode::Right => t0.offset((1, 0))?,
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
                nc.render()?;
            }
        }
        l.sleep(Duration::microseconds(1));
    }

    info!["bye!"];
    Ok(())
}

#[cfg(feature = "gilrs")]
fn input_gilrs(gilrs: &mut GilrsBackend) -> Result<()> {
    let event = gilrs.poll_event()?;
    match event.kind {
        EventKind::Gamepad(g) => {
            debug!["gilrs: {g:?}"];
            debug!["counter: {:?}", gilrs.counter()];
        }
        _ => (),
    }
    gilrs.increment();
    Ok(())
}

#[cfg(feature = "midir")]
fn input_midir(midir: &mut MidirBackend) -> Result<()> {
    let event = midir.poll_event()?;
    let t = event.emitted.unwrap_or_default();
    match event.kind {
        EventKind::Midi(m) => {
            trace!["midir [{t}]: {m:?}"];
        }
        _ => (),
    }
    Ok(())
}

// TODO: needs control messaging system
// #[cfg(feature = "notcurses")]
// fn input_notcurses(nc: &mut NotcursesBackend) -> Result<()> {
//     Ok(())
// }
