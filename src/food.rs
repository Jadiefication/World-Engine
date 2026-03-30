use crate::entity::Entity;
use crate::world::World;

pub struct Food {
    pos: (usize, usize),
    eaten: bool,
    growth_stage: u8,
    mold_level: f32
}

impl Entity for Food {
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