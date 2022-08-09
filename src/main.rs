mod audio;
mod def_const;
mod def_plugins;
mod input_keyboard;
mod vis;
mod vis_console;
use crate::{def_plugins::*, vis_console::VisConsole};

fn main() {
    // INIT Plugins
    let plugins = Plugins {
        input: input_keyboard::InputHandler::new(),
        vis: VisConsole::new(),
    };

    // INIT SOUND
    let audio_tx = audio::metro_audio_init();
    let sound_on = true;

    // Init LINK:
    let mut link = ableton_link::Link::new(120.0);
    link.enable(true);

    let link_enabled = true;
    link.enable_start_stop_sync(true);

    let clock = link.clock();
    let quantum = 4.0;

    let mut tempo: f64 = 0.0;

    // Remember prev values:
    let mut last_tempo: f64 = 0.0;
    let mut last_beat: f64 = 0.0;

    // Init Values
    link.with_app_session_state(|ss| {
        tempo = ss.tempo();
        last_tempo = tempo;
    });

    // VISUALS options:
    // let mut vis_on = true;

    // ----------------------------------------------------------------------------------------------------------------
    // MAIN LOOP
    loop {
        // Poll Input
        if let Some(x) = plugins.input.poll() {
            println!("{:?}", x)
        }

        // GET CURRENT SESSION STATE:
        link.with_app_session_state(|session_state| {
            tempo = session_state.tempo();
            let time = clock.micros();
            let beat = session_state.beat_at_time(time, quantum);
            let phase = session_state.phase_at_time(time, quantum);

            let _peers = link.num_peers();
            let _playing = session_state.is_playing();

            // println!(
            //     "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
            //     _playing, quantum, tempo, beat, phase, _peers
            // );

            // EVERY FULL BEAT:
            if beat - last_beat >= 1.0 {
                last_beat = beat.floor(); // re-calibrate to full beat

                // new_color_on_beat = vis::RGB8::new_rnd(); // change this color value every beat

                // Trigger Sound:
                if sound_on {
                    match phase.floor() as i32 {
                        0 => audio_tx.send(1).unwrap(), // on the first beat
                        _ => audio_tx.send(0).unwrap(), // on any other beat
                    }
                }
            }
        });

        // UPDATE LINK WITH CONTROL CHANGES:
        if link_enabled {
            link.enable(true);
        } else {
            link.enable(false);
        }
        if !last_tempo.eq(&tempo) {
            link.with_app_session_state(|mut ff| {
                ff.set_tempo(tempo, clock.micros());
                ff.commit();
            });
            last_tempo = tempo;
        }
    }
}
