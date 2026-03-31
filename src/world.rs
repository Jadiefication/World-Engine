use std::cell::RefCell;
use std::collections::HashMap;
use crate::entity::{Entity, EntityId};

pub struct World {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    entity_posistions: HashMap<(usize, usize), EntityId>,
    size: (u32, u32),
    entity_queue: Vec<(Box<dyn Entity>, (usize, usize))>
}

impl World {
    pub fn new() -> World {
        World { entities: HashMap::new(), entity_posistions: HashMap::new(), size: (256, 256), entity_queue: vec![] }
    }

    pub fn from(entities: HashMap<EntityId, Box<dyn Entity>>, entity_posistions: HashMap<(usize, usize), EntityId>, size: (u32, u32)) -> World {
        World { entities, entity_posistions, size, entity_queue: vec![] }
    }

    fn process(&mut self) {
        let mut entities = std::mem::take(&mut self.entities);

        for entity in &mut entities {
            let pos = self.entity_posistions.iter().find(|pos| { *pos.1 == *entity.0 }).unwrap();
            entity.1.process(self, *pos.0)
        }
        self.entities = entities;
        for (entity, pos) in self.entity_queue.drain(..) {
            self.entity_posistions.insert(pos, entity.get_id());
            self.entities.insert(entity.get_id(), entity);
        }
    }

    pub fn spawn_entity(&mut self, entity: Box<dyn Entity>, pos: (usize, usize)) {
        self.entity_queue.push((entity, pos))
    }

    pub fn find_by_id(&self, id: EntityId) -> Option<&Box<dyn Entity>> {
        self.entities.get(&id)
    }
}