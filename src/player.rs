use std::any::Any;
use crate::entity::{Entity, EntityId};
use crate::world::World;

pub struct Player {
    pos: (usize, usize),
    food_level: f32,
    id: EntityId
}

impl Player {
    fn check_for_food(&self, world: &World) -> (usize, usize) {
        todo!()
    }
}

impl Entity for Player {
    fn pos(&self) -> (usize, usize) {
        todo!()
    }

    fn set_pos(&mut self, pos: (usize, usize), world: &mut World) -> (usize, usize) {
        todo!()
    }

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