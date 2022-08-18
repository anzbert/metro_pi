mod audio;
mod def_const;
mod def_plugins;
mod def_settings;
mod gifs;
mod input_keyboard;
mod utilities;
mod vis_led;

use crate::def_plugins::*;
use def_settings::Settings;
use gif2json::RgbaImageData;
use gifs::Visualization;
use vis_led::VisLed;

fn main() {
    // PLUGINS
    let input_plugin = input_keyboard::InputHandler::new();
    let mut vis_plugin = VisLed::new();

    // SETTINGS
    let mut settings = Settings {
        visual: Visualization::default(),
        brightness: 100,
        sound_enabled: true,
        volume: 100,
        link_enabled: true,
        tempo: 120.0,
        quantum: 4.0,
    };

    // INIT SOUND
    let audio_tx = audio::metro_audio_init();

    // INIT LINK:
    let mut link = ableton_link::Link::new(120.0);
    let clock = link.clock();
    link.enable(true);
    link.enable_start_stop_sync(true);

    // Init link buffer values
    let mut last_tempo: f64 = 0.0;
    let mut last_beat: f64 = 0.0;

    // Get Startup Link Values
    link.with_app_session_state(|ss| {
        settings.tempo = ss.tempo();
        last_tempo = settings.tempo;
    });

    // ----------------------------------------------------------------------------
    // MAIN LOOP
    loop {
        // POLL INPUT
        if let Some(x) = input_plugin.poll() {
            println!("{:?}", x)
        }

        // GET CURRENT SESSION STATE:
        link.with_app_session_state(|session_state| {
            settings.tempo = session_state.tempo();
            let time = clock.micros();
            let beat = session_state.beat_at_time(time, settings.quantum);
            let phase = session_state.phase_at_time(time, settings.quantum);
            let _peers = link.num_peers();
            let _playing = session_state.is_playing();

            // println!(
            //     "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
            //     _playing, quantum, tempo, beat, phase, _peers
            // );

            // EVERY FULL BEAT:
            if beat - last_beat >= 1.0 {
                last_beat = beat.floor(); // zero to last full beat

                // DRAW OUTPUT:
                vis_plugin.update(settings.quantum, phase.floor());

                // TRIGGER SOUND:
                if settings.sound_enabled {
                    match phase.floor() as u32 {
                        0 => audio_tx.send(1).unwrap(), // on the first beat
                        _ => audio_tx.send(0).unwrap(), // on any other beat
                    }
                }
            }
        });

        // UPDATE LINK WITH CONTROL CHANGES:
        if settings.link_enabled {
            link.enable(true);
        } else {
            link.enable(false);
        }
        if !last_tempo.eq(&settings.tempo) {
            link.with_app_session_state(|mut session_state| {
                session_state.set_tempo(settings.tempo, clock.micros());
                session_state.commit();
            });
            last_tempo = settings.tempo;
        }
    }
}
