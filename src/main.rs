mod audio;
mod constants;
mod input;
mod vis;

use gif2json::RgbaImageData;

fn main() {
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

    // Init VISUALS:
    let mut vis_on = true;

    #[derive(PartialEq, Debug)]
    enum Vis {
        Off,
        One,
        Two,
        Three,
        Four,
    }
    let mut vis_selected = Vis::One;

    let mut vis_numbers = true;

    let mut leds = vis::Leds::new();
    let mut new_color_on_beat = vis::RGB8::new_rnd();

    // Init the GIFs:
    let gif_counter =
        RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap();
    let gif_clock = RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap();
    let gif_rows = RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap();
    let gif_circular =
        RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap();

    // ----------------------------------------------------------------------------------------------------------------
    // MAIN LOOP
    loop {
        // GET KEYBOARD INPUT
        // input::check_input();

        // GET CURRENT SESSION STATE:
        link.with_app_session_state(|session_state| {
            tempo = session_state.tempo();
            let time = clock.micros();
            let beat = session_state.beat_at_time(time, quantum);
            let phase = session_state.phase_at_time(time, quantum);

            let peers = link.num_peers();
            let play = session_state.is_playing();

            println!(
                "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
                play, quantum, tempo, beat, phase, peers
            );

            // EVERY FULL BEAT:
            if beat - last_beat >= 1.0 {
                last_beat = beat.floor(); // re-calibrate to full beat

                // new_color_on_beat = vis::RGB8::new_rnd(); // change this color value every beat

                // Trigger Sound:
                if sound_on {
                    match phase.floor() as i32 {
                        0 => audio_tx.send(1).unwrap(),
                        _ => audio_tx.send(0).unwrap(),
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
