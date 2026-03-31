use std::collections::HashMap;
use rand::RngExt;
use crate::entity::Entity;
use crate::world::World;

pub struct Food {
    pos: (usize, usize),
    eaten: bool,
    growth_stage: i8,
    mold_level: f32,
    family: HashMap<(usize, usize), Food>,
}

impl Food {
    fn new() -> Food {
        Food {
            pos: (0, 0),
            eaten: false,
            growth_stage: 0,
            mold_level: 0.0,
            family: HashMap::new()
        }
    }

    fn grow(&mut self, world: &mut World) {
        let dirs = [(1isize,0),(-1,0),(0,1),(0,-1)];
        let (dx, dy) = dirs[rand::rng().random_range(0..4)];
        let new_pos = (
            (self.pos.0 as isize + dx).max(0) as usize,
            (self.pos.1 as isize + dy).max(0) as usize
        );
        self.family.insert(new_pos, Self::new());
    }

    fn die(&mut self) {
        self.growth_stage = -1
    }
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