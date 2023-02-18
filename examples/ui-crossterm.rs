// revela::example::ui-crossterm
//
//! # `crossterm` UI example
//!
//! Optionally supported backends: `gilrs`, `midir`.
//
// IMPROVE
// - allow non-raw-mode + command to enter and leave.
// - manage signals while in raw mode.

use depura::*;
use repite::*;
use revela::all::{RevelaResult as Result, *};

fn main() -> Result<()> {
    Logger::new("revela example ui-crossterm")
        .file("log-uis.log")
        .target_level_all()
        //
        .ignore("gilrs::ff")
        .ignore("gilrs_core")
        //
        .init()
        .unwrap();
    info!["press Escape to exit"];

    let mut _crossterm = CrosstermBackend::new()?;
    let ct = &mut _crossterm;

    ct.enable_bracketed_paste()?;
    ct.enter_alternate_screen()?;
    ct.enable_raw_mode()?;
    info!["raw_mode:{}", ct.is_raw_mode()?];

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

                // TODO: exit with ctrl-c or other key
                // - https://github.com/crossterm-rs/crossterm/issues/554#issuecomment-857001853
                let ct_event = ct.poll_event()?;
                match ct_event.kind {
                    EventKind::Window(w) => {
                        debug!["window: {w:?}"];
                    }
                    EventKind::Key(key) => {
                        debug!["key: {key:?}"];
                        match key.code {
                            KeyCode::Escape | KeyCode::Char('q') => {
                                ct.disable_raw_mode()?;
                                // ct.leave_alternate_screen()?;
                                break;
                            }
                            KeyCode::Char('l') => l.log_all_rates(),
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
                // TODO
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
