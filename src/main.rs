pub mod entity;
pub mod world;
pub mod player;
pub mod food;
pub mod obstacle;
pub mod event;

use std::sync::mpsc;
use crossterm::terminal::enable_raw_mode;
use crate::event::event::Event::Exit;
use crate::event::listener::Listener;
use crate::world::World;

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");
    let (tx, rx) = mpsc::channel();

    Listener::start(tx);

    loop {
        if let Ok(event) = rx.try_recv() {
            if event == Exit {
                break
            } else {
                World::new().process(event)
            }
        }
    }
}
