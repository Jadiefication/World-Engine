use std::time::Duration;
use crate::entity::{Entity, EntityId};
use crate::world::World;

pub struct Obstacle {
    pos: (usize, usize),
    durability: f32,
    duration: Option<Duration>,
    id: EntityId
}

impl Entity for Obstacle {
    fn pos(&self) -> (usize, usize) {
        todo!()
    }

    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        todo!()
    }

    fn process(&mut self, world: &World, xy: (usize, usize)) {
        todo!()
    }

    fn get_id(&self) -> EntityId {
todo!()
    }
}