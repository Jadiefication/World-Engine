use std::any::Any;
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
    fn process(&mut self, world: &mut World) {
        todo!()
    }

    fn get_id(&self) -> EntityId {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }
}