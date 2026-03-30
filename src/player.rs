use crate::entity::Entity;
use crate::world::World;

pub struct Player {
    pos: (usize, usize),
    food_level: f32
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

    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        todo!()
    }

    fn process(&mut self, world: &World, xy: (usize, usize)) {
        todo!()
    }
}