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
                    Event::Key(event) => match event.code {
                        KeyCode::Char('q') => std::process::exit(0),
                        _ => last_pressed = Some(event.code),
                    },
                    Event::Resize(_width, _height) => {
                        let mut stdout = std::io::stdout();
                        stdout
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                    }
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
                        KeyCode::Right => Some(Input::Right),
                        KeyCode::Left => Some(Input::Left),
                        KeyCode::Char(' ') => Some(Input::Button),
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
