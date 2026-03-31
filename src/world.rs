use crate::entity::{Entity, EntityId};
use std::collections::HashMap;

pub struct World {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    pub entity_positions: HashMap<(usize, usize), EntityId>,
    size: (u32, u32),
    entity_queue: Vec<(Box<dyn Entity>, (usize, usize))>,
    pub time: f32
}

impl World {
    pub fn new() -> World {
        World { entities: HashMap::new(), entity_positions: HashMap::new(), size: (256, 256), entity_queue: vec![], time: 0.0 }
    }

    pub fn from(entities: HashMap<EntityId, Box<dyn Entity>>, entity_positions: HashMap<(usize, usize), EntityId>, size: (u32, u32)) -> World {
        World { entities, entity_positions, size, entity_queue: vec![], time: 0.0 }
    }

    fn process(&mut self) {
        let mut entities = std::mem::take(&mut self.entities);

        for entity in &mut entities {
            self.entity_positions.iter().find(|pos| { *pos.1 == *entity.0 }).unwrap();
            entity.1.process(self)
        }
        self.entities = entities;
        for (entity, pos) in self.entity_queue.drain(..) {
            self.entity_positions.insert(pos, entity.get_id());
            self.entities.insert(entity.get_id(), entity);
        }
    }

    pub fn spawn_entity(&mut self, entity: Box<dyn Entity>, pos: (usize, usize)) {
        self.entity_queue.push((entity, pos))
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Box<dyn Entity>> {
        self.entities.get(&id)
    }

    pub(crate) fn find_by_id_mut(&mut self, id: EntityId) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: EntityId) -> Box<dyn Entity> {
        let old_entity = self.entities.remove(&id).unwrap();
        self.entity_positions.remove(&old_entity.pos());
        old_entity
    }
}