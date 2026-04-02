use std::any::Any;
use crate::entity::{Entity, EntityId, Pos};
use crate::world::World;
use rand::{Rng, RngExt};
use std::collections::HashMap;
use crate::event::event::Event;

pub struct Food {
    pos: Pos,
    growth_stage: u8,
    max_growth: u8,
    mold_level: f32,
    family: HashMap<Pos, EntityId>,
    id: EntityId,
    removed_entities: Vec<Pos>
}

impl Food {
    pub fn new() -> Food {
        Food {
            pos: (0, 0),
            growth_stage: 0,
            max_growth: 5,
            mold_level: 0.0,
            family: HashMap::new(),
            id: (rand::rng().next_u32() as i32) as EntityId,
            removed_entities: vec![]
        }
    }

    pub fn from(
        pos: Pos,
        growth_stage: u8,
        max_growth: u8,
        mold_level: f32
    ) -> Food {
        Food {
            pos,
            growth_stage,
            max_growth,
            mold_level,
            family: HashMap::new(),
            id: (rand::rng().next_u32() as i32) as EntityId,
            removed_entities: vec![]
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

    fn die(&mut self, world: &mut World) {
        let ids: Vec<EntityId> = self.family.values().copied().collect();

        for id in ids {
            if let Some(food) = world.find_by_id_mut(id) {
                if let Some(ent) = food.as_any_mut().downcast_mut::<Food>() {
                    ent.mark_removed_member(&self.pos);
                }
            }
        }
        world.mark_removed_entity(self.id);
    }

    fn mark_removed_member(&mut self, pos: &Pos) {
        self.removed_entities.push(*pos)
    }
}

impl Entity for Food {
    fn process(&mut self, world: &mut World, event: &Event) {
        if self.growth_stage == self.max_growth {
            self.mold_level = 2.0_f32.powf((0.56 * world.time).powf(0.1 * world.time));
            let rand = rand::rng().next_u32();
            if rand % 4 == 0 {
                self.grow(world);
            }
            if rand % 11 == 0 {
                self.die(world)
            }
        } else {
            self.growth_stage += 1;
        }
        for pos in self.removed_entities.drain(..) {
            self.family.remove(&pos);
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