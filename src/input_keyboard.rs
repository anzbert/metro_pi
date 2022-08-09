use std::io::stdin;

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::def_plugins::*;

pub struct InputHandler {
    poll_tx: Sender<String>,
    poll_rx: Receiver<Option<String>>,
}

impl InputPlugin for InputHandler {
    fn new() -> Self {
        let (poll_tx, other_thread_rx): (Sender<String>, Receiver<String>) =
            std::sync::mpsc::channel();
        let (other_thread_tx, poll_rx): (Sender<Option<String>>, Receiver<Option<String>>) =
            std::sync::mpsc::channel();

        thread::spawn(move || {
            let mut guess = String::new();
            stdin().read_line(&mut guess).expect("Failed to read line");

            for message in other_thread_rx {
                match message {
                    _ => {
                        if !guess.is_empty() {
                            other_thread_tx
                                .send(Some(guess.clone().trim().to_string()))
                                .unwrap();
                            other_thread_tx.send(None).unwrap();
                            guess.clear();

                            stdin().read_line(&mut guess).expect("Failed to read line");
                        }
                    }
                };
            }
        });

        InputHandler { poll_tx, poll_rx }
    }

    fn poll(&self) -> Option<Input> {
        self.poll_tx.send("poll".to_string()).unwrap();

        match self.poll_rx.try_recv() {
            Ok(x) => match x {
                Some(x) => match x.as_str() {
                    "r" => Some(Input::new(false, true, false)),
                    "l" => Some(Input::new(true, false, false)),
                    "b" => Some(Input::new(false, false, true)),
                    _ => {
                        println!("--> Only acceptable input is 'l', 'r', or 'b'");
                        None
                    }
                },
                None => None,
            },
            Err(_) => None,
        }
    }
}
