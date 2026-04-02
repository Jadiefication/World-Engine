use crate::event::event::Event as GEvent;
use crate::event::event::Event::{Eat, Exit, Movement};
use crate::player::Direction::{Down, Left, Right, Up};
use crossterm::event::{self, Event, KeyCode};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub struct Listener;

impl Listener {
    pub fn start(tx: Sender<GEvent>) {
        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(50)).unwrap() {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        let ev = match key_event.code {
                            KeyCode::Char('w') => Some(Movement(Up)),
                            KeyCode::Char('s') => Some(Movement(Down)),
                            KeyCode::Char('a') => Some(Movement(Left)),
                            KeyCode::Char('d') => Some(Movement(Right)),
                            KeyCode::Char('e') => Some(Eat),
                            KeyCode::Esc => Some(Exit),
                            _ => None,
                        };

                        if let Some(ev) = ev {
                            tx.send(ev).unwrap();
                        }
                    }
                }
            }
        });
    }
}