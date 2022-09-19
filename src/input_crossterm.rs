use crate::def_input::Input;
use crate::def_plugins::*;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{self};
use crossterm::ExecutableCommand;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self};

pub struct InputCrossterm {
    poll_tx: Sender<Poke>,
    poll_rx: Receiver<Option<KeyCode>>,
}

struct Poke;

impl InputPlugin for InputCrossterm {
    fn new() -> Self {
        let (poll_tx, other_thread_rx): (Sender<Poke>, Receiver<Poke>) = std::sync::mpsc::channel();
        let (other_thread_tx, poll_rx): (Sender<Option<KeyCode>>, Receiver<Option<KeyCode>>) =
            std::sync::mpsc::channel();

        thread::spawn(move || {
            crossterm::terminal::enable_raw_mode().unwrap();

            let mut last_pressed: Option<KeyCode> = None;

            for message in other_thread_rx {
                match read().unwrap() {
                    // Event::Key(event) => match event.code {
                    //     KeyCode::Char('q') => std::process::exit(0),
                    //     _ => last_pressed = Some(event.code),
                    // },
                    Event::Resize(_width, _height) => {
                        let mut stdout = std::io::stdout();
                        stdout
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                    }
                    // Event::Mouse(MouseEventKind::ScrollUp) => (),
                    _ => (),
                };
                match message {
                    _ => {
                        if last_pressed != None {
                            other_thread_tx.send(last_pressed).unwrap();
                            last_pressed = None;
                        }
                    }
                };
            }
        });

        InputCrossterm { poll_tx, poll_rx }
    }

    fn poll(&mut self) -> Option<Input> {
        self.poll_tx.send(Poke).unwrap();

        match self.poll_rx.try_recv() {
            Ok(x) => match x {
                Some(x) => {
                    match x {
                        KeyCode::Char('q') => Some(Input::Quit),
                        KeyCode::Up => Some(Input::Up),
                        KeyCode::Down => Some(Input::Down),
                        KeyCode::Right => Some(Input::Right),
                        KeyCode::Left => Some(Input::Left),
                        KeyCode::Char(' ') => Some(Input::Button),
                        KeyCode::Char('1') => Some(Input::Volume(0.)),
                        KeyCode::Char('2') => Some(Input::Volume(0.125)),
                        KeyCode::Char('3') => Some(Input::Volume(0.25)),
                        KeyCode::Char('4') => Some(Input::Volume(0.375)),
                        KeyCode::Char('5') => Some(Input::Volume(0.5)),
                        KeyCode::Char('6') => Some(Input::Volume(0.625)),
                        KeyCode::Char('7') => Some(Input::Volume(0.75)),
                        KeyCode::Char('8') => Some(Input::Volume(0.875)),
                        KeyCode::Char('9') => Some(Input::Volume(1.)),
                        _ => {
                            // println!("--> Only acceptable input is 'left arrow', 'right arrow', 'space' or 'q' to quit");
                            None
                        }
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
