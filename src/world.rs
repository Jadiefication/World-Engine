use crate::entity::{Entity, EntityId, Pos};
use std::collections::HashMap;

pub struct World {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    pub entity_positions: HashMap<Pos, EntityId>,
    size: (u32, u32),
    entity_queue: Vec<(Box<dyn Entity>, (usize, usize))>,
    pub time: f32,
    remove_entity_queue: Vec<EntityId>
}

impl World {
    pub fn new() -> World {
        World { entities: HashMap::new(), entity_positions: HashMap::new(), size: (256, 256), entity_queue: vec![], time: 0.0, remove_entity_queue: vec![] }
    }

    pub fn from(entities: HashMap<EntityId, Box<dyn Entity>>, entity_positions: HashMap<Pos, EntityId>, size: (u32, u32)) -> World {
        World { entities, entity_positions, size, entity_queue: vec![], time: 0.0, remove_entity_queue: vec![] }
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
        for id in self.remove_entity_queue.drain(..) {
            self.entities.remove(&id);
            
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

    pub fn mark_removed_entity(&mut self, id: EntityId) {
        self.remove_entity_queue.push(id)
    }
    
    pub fn update_pos(&mut self, id: EntityId, new_pos: Pos) {
        self.entity_positions.iter_mut().find(|it| {
            *it.1 == id
        }).unwrap().0 = &new_pos;
    }
}