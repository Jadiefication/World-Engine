use std::any::Any;
use crate::entity::{Entity, EntityId};
use crate::world::World;
use rand::{Rng, RngExt};
use std::collections::HashMap;

pub struct Food {
    pos: (usize, usize),
    growth_stage: i8,
    max_growth: i8,
    mold_level: f32,
    family: HashMap<(usize, usize), EntityId>,
    id: EntityId
}

impl Food {
    fn new() -> Food {
        Food {
            pos: (0, 0),
            growth_stage: 0,
            max_growth: 5,
            mold_level: 0.0,
            family: HashMap::new(),
            id: (rand::rng().next_u32() as i32) as EntityId
        }
    }

    fn grow(&mut self, world: &mut World) -> EntityId {
        let dirs = [(1isize,0),(-1,0),(0,1),(0,-1)];
        let (dx, dy) = dirs[rand::rng().random_range(0..4)];
        let new_pos = (
            (self.pos.0 as isize + dx).max(0) as usize,
            (self.pos.1 as isize + dy).max(0) as usize
        );
        let mut entity = Box::from(Self::new());
        entity.family.insert(self.pos, self.id);
        let id = entity.id;
        self.family.insert(new_pos, entity.id);
        world.spawn_entity(entity, new_pos);
        id
    }

    fn die(&mut self) {
        self.growth_stage = -1
    }
}

impl Entity for Food {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }

    fn set_pos(&mut self, pos: (usize, usize), world: &mut World) -> (usize, usize) {
        let old_pos = self.pos;
        self.pos = pos;
        world.entity_positions.remove(&old_pos);
        world.entity_positions.insert(pos, self.id);
        self.family.iter().for_each(|entity| {
            let food = world.find_by_id_mut(*entity.1).unwrap();
            let ent = food.as_any_mut().downcast_mut::<Food>().unwrap();
            ent.family.remove(&old_pos);
            ent.family.insert(pos, self.id);
        });
        old_pos
    }

    fn process(&mut self, world: &mut World) {
        if self.growth_stage != -1 {
            if self.growth_stage == self.max_growth {
                self.mold_level = 2.0_f32.powf((0.56 * world.time).powf(0.1 * world.time));
                let rand = rand::rng().next_u32();
                if rand % 4 == 0 {
                    let id = self.grow(world);
                }
                if rand % 11 == 0 {
                    self.die()
                }
            } else {
                self.growth_stage += 1;
            }
        } else {
            let ids: Vec<EntityId> = self.family.values().copied().collect();

            for id in ids {
                if let Some(food) = world.find_by_id_mut(id) {
                    if let Some(ent) = food.as_any_mut().downcast_mut::<Food>() {
                        ent.family.remove(&self.pos);
                    }
                }
            }
            world.remove_entity(self.id);
        }
    }

    fn get_id(&self) -> EntityId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}