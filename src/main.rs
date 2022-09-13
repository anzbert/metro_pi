mod animations;
mod audio;
mod def_const;
mod def_grid;
mod def_input;
mod def_plugins;
mod def_settings;
mod font;
// #[cfg(any(target_os = "macos", target_os = "windows"))]
mod input_crossterm;
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
mod input_hardware;
mod input_null;
mod utilities;
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod vis_crossterm;
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
mod vis_led;
mod vis_null;
use animations::{METRO_ANIMATIONS, TEXT_ANIMATIONS};
use audio::Sound;
use core::time;
use def_const::SOUND_PATHS;
use def_input::Input;
use def_plugins::*;
use def_settings::Settings;
use rodio::OutputStream;
use std::time::Instant;

fn main() {
    // SETTINGS
    let mut settings: Settings = Settings {
        visual: METRO_ANIMATIONS.get(0).unwrap(),
        brightness: 3,
        link_enabled: true,
        tempo: 120.0,
        quantum: 4.0,
    };

    ////////////////////////////////////////////////////////////////////

    // PLUGINS
    // #[allow(unused_variables, unused_mut)]
    // let mut vis_plugin = vis_null::VisNull::new(settings.visual, settings.brightness);
    // #[allow(unused_variables, unused_mut)]
    // let mut input_plugin = input_null::InputNull::new();

    // #[cfg(any(target_os = "macos", target_os = "windows"))]
    let mut input_plugin = input_crossterm::InputCrossterm::new();

    // #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    // let mut input_plugin = input_hardware::InputHardware::new();

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    let mut vis_plugin = vis_led::VisLed::new(settings.visual, settings.brightness);

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let mut vis_plugin = vis_crossterm::VisCrossterm::new(settings.visual, settings.brightness);

    ////////////////////////////////////////////////////////////////////

    // INIT GIFS
    let mut current_vis_index: usize = 0;

    // Init Sound Device
    let mut audio_player = audio::AudioPlayer::new();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sound0 = Sound::new(SOUND_PATHS[0]);
    let sound1 = Sound::new(SOUND_PATHS[1]);
    // let test_music = Sound::new(SOUND_PATHS[2]);

    // Init Link
    let mut link = ableton_link::Link::new(settings.tempo);
    let clock = link.clock();
    link.enable(true);
    link.enable_start_stop_sync(true);

    let mut last_tempo: f64 = settings.tempo;
    let mut last_beat: f64 = 0.0;

    // DELTA TIME
    let mut d_time = Instant::now();

    // ---------------------------------------------------------------------------- //
    #[allow(unused_labels)]
    'main: loop {
        // THREAD SLEEP (to save cpu usage)
        spin_sleep::sleep(time::Duration::from_millis(5));

        // POLL INPUT
        if let Some(input) = input_plugin.poll() {
            // println!("Debug Received: {:?}", x);

            match input {
                Input::Volume(x) => {
                    audio_player.set_volume(x);
                }
                Input::Left => {
                    current_vis_index = match current_vis_index.checked_sub(1) {
                        Some(x) => x,
                        None => TEXT_ANIMATIONS.len() - 1,
                    };
                    vis_plugin.select_metro_loop(METRO_ANIMATIONS.get(current_vis_index).unwrap());
                }
                Input::Right => {
                    current_vis_index = (current_vis_index + 1) % METRO_ANIMATIONS.len();
                    vis_plugin.select_metro_loop(METRO_ANIMATIONS.get(current_vis_index).unwrap());
                }
                Input::Button => {
                    vis_plugin.select_single_play(TEXT_ANIMATIONS.get(0).unwrap());
                }
                _ => (),
            }
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
                if audio_player.get_volume() > 0. {
                    match phase.floor() as u32 {
                        0 => audio_player.play(&stream_handle, &sound1), // on the first beat
                        _ => audio_player.play(&stream_handle, &sound0), // on any other beat
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

        // Update delta-time
        d_time = Instant::now();
    }
}
