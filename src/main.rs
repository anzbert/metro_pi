mod audio;
mod def_const;
mod def_plugins;
mod def_settings;
mod gifs;
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod input_keyboard;
mod input_null;
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
mod input_pins;
mod utilities;
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
mod vis_led;
mod vis_null;
use core::time;
use std::thread;

use crate::def_plugins::*;
use def_settings::Settings;
use gifs::Visualization;
use gifs::GIFS;

use vis_null::VisNull;

fn main() {
    // SETTINGS
    let mut settings: Settings = Settings {
        visual: Visualization::default(),
        brightness: 3,
        sound_enabled: false,
        volume: 100, // todo
        link_enabled: true,
        tempo: 120.0,
        quantum: 4.0,
    };

    // PLUGINS
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let mut input_plugin = input_keyboard::InputKeyboard::new();

    #[cfg(target_os = "unix")]
    let mut input_plugin = input_pins::InputPins::new();

    #[cfg(target_os = "unix")]
    let mut vis_plugin = vis_led::VisLed::new(settings.visual, settings.brightness);

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let mut vis_plugin = VisNull::new(settings.visual, settings.brightness);

    // Init GIF Collection
    let all_vis: Vec<&Visualization> = GIFS.keys().collect();
    let mut current_vis_index: usize = 0;

    // Init Sound Device
    let audio_tx = audio::metro_audio_init();

    // Init Link
    let mut link = ableton_link::Link::new(settings.tempo);
    let clock = link.clock();
    link.enable(true);
    link.enable_start_stop_sync(true);

    let mut last_tempo: f64 = settings.tempo;
    let mut last_beat: f64 = 0.0;

    // ---------------------------------------------------------------------------- //
    #[allow(unused_labels)]
    'main: loop {
        // THREAD SLEEP (to save cpu usage)
        thread::sleep(time::Duration::from_millis(8));

        // POLL INPUT
        if let Some(x) = input_plugin.poll() {
            // println!("received: {:?}", x);
            // current_vis_index = (current_vis_index + 1) % (all_vis.len() - 1);

            // vis_plugin.select(**all_vis.get(current_vis_index).unwrap());
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
            //     _playing,
            //     settings.quantum,
            //     session_state.tempo(),
            //     beat,
            //     phase,
            //     _peers
            // );

            // DRAW OUTPUT:
            vis_plugin.update(settings.quantum, phase);

            // EVERY FULL BEAT:
            if beat - last_beat >= 1.0 {
                last_beat = beat.floor(); // zero to last full beat

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
