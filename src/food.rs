use crate::entity::{Entity, EntityId};
use crate::world::World;
use rand::{Rng, RngExt};
use std::collections::HashMap;

pub struct Food {
    pos: (usize, usize),
    eaten: bool,
    growth_stage: i8,
    mold_level: f32,
    family: HashMap<(usize, usize), EntityId>,
    id: EntityId
}

impl Food {
    fn new() -> Food {
        Food {
            pos: (0, 0),
            eaten: false,
            growth_stage: 0,
            mold_level: 0.0,
            family: HashMap::new(),
            id: (rand::rng().next_u32() as i32) as EntityId
        }
    }

    fn grow(&mut self, world: &mut World) {
        let dirs = [(1isize,0),(-1,0),(0,1),(0,-1)];
        let (dx, dy) = dirs[rand::rng().random_range(0..4)];
        let new_pos = (
            (self.pos.0 as isize + dx).max(0) as usize,
            (self.pos.1 as isize + dy).max(0) as usize
        );
        let entity = Box::from(Self::new());
        self.family.insert(new_pos, entity.id);
        world.spawn_entity(entity, new_pos);
    }

    fn die(&mut self) {
        self.growth_stage = -1
    }
}

impl Entity for Food {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }

    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        let old_pos = self.pos.clone();
        self.pos = pos;
        old_pos
    }

    fn process(&mut self, world: &mut World, xy: (usize, usize)) {
        todo!()
    }

    fn get_id(&self) -> EntityId {
        self.id
    }
}