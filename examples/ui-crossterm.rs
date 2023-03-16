// revela::examples::ui-crossterm
//
//! # `crossterm` UI example
//!
//! Optionally supported backends: `gilrs`, `midir`.
//!
// run on debug or release, respectively with:
// * cargo re_all ui-crossterm
// * cargo rre_all ui-crossterm

use depura::*;
use devela::crate_root_string;
use repite::*;
use revela::all::{RevelaResult as Result, *};

#[cfg(feature = "kira")]
use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

fn main() -> Result<()> {
    Logger::new("revela example ui-crossterm")
        .file(&crate_root_string("examples/log-uis.log"))
        .target_level_all()
        //
        .ignore("gilrs::ff")
        .ignore("gilrs_core")
        //
        .init()
        .unwrap();
    info!["press 'q' to exit"];

    let mut _crossterm = CrosstermBackend::new()?;
    let ct = &mut _crossterm;

    info!["capabilities: {:?}", ct.capabilities()];

    ct.enable_mouse()?;
    ct.enable_bracketed_paste()?;
    ct.enable_focus_change()?;
    ct.enter_alternate_screen()?;
    ct.enable_raw_mode()?;

    /* */

    #[cfg(feature = "gilrs")]
    let mut gilrs = GilrsBackend::new()?;

    #[cfg(feature = "midir")]
    let mut midir = {
        let mut midir = MidirBackend::new()?;
        midir.in_connect_all()?;
        debug!["{midir:?}"];
        midir
    };

    /* sound */

    #[cfg(feature = "kira")]
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
    #[cfg(feature = "kira")]
    let sound0 = StaticSoundData::from_file(
        &crate_root_string("examples/sound.ogg"),
        StaticSoundSettings::default(),
    )?;

    /* loop */

    let mut l = Looper::new();
    assert![l.add_rate("input", Rate::with_tps(200.), true).is_ok()];
    assert![l.add_rate("render", Rate::with_tps(24.), true).is_ok()];
    l.reset();

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
                            KeyCode::Char('q') => {
                                ct.disable_raw_mode()?;
                                // ct.leave_alternate_screen()?;
                                break;
                            }
                            KeyCode::Char('l') => l.log_all_rates(),

                            /* sound */
                            #[cfg(feature = "kira")]
                            KeyCode::Char('a') => {
                                info!["[kira] play sound0"];
                                let _h = manager.play(sound0.clone())?;
                            }

                            _ => (),
                        }
                    }
                    EventKind::Mouse(m) => {
                        debug!["mouse: {m:?}"];
                    }
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
