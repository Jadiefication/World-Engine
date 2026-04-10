use crate::entity::{Entity, EntityId, Pos};
use std::collections::HashMap;
use std::time::Duration;
use crate::player::Player;

pub struct World {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    pub size: (usize, usize),
    entity_queue: Vec<(Box<dyn Entity>, (usize, usize))>,
    pub time: f32,
    remove_entity_queue: Vec<EntityId>
}

pub struct WorldContext<'a> {
    entities: &'a HashMap<EntityId, Box<dyn Entity>>,
    size: (usize, usize),
    time: f32
}

impl World {
    pub fn new() -> World {
        World { entities: HashMap::new(), size: (256, 256), entity_queue: vec![], time: 0.0, remove_entity_queue: vec![] }
    }

    pub fn from(entities: HashMap<EntityId, Box<dyn Entity>>, size: (usize, usize)) -> World {
        World { entities, size, entity_queue: vec![], time: 0.0, remove_entity_queue: vec![] }
    }

    pub fn process(&mut self) {
        let ids: Vec<_> = self.entities.keys().cloned().collect();

        for id in ids {
            let mut entity = self.entities.remove(&id).unwrap();

            let mut view = WorldContext {
                entities: &self.entities,
                size: self.size,
                time: self.time,
            };

            entity.process(&mut view);

            self.entities.insert(id, entity);
        }
        for (entity, pos) in self.entity_queue.drain(..) {
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

    pub fn find_by_id_mut(&mut self, id: EntityId) -> Option<&mut Box<dyn Entity>> {
        self.entities.get_mut(&id)
    }

    pub fn find_by_pos(&self, pos: Pos) -> Option<&Box<dyn Entity>> {
        self.entities.values().find(|it| it.get_pos() == pos)
    }

    pub fn find_by_pos_mut(&mut self, pos: Pos) -> Option<&mut Box<dyn Entity>> {
        self.entities.values_mut().find(|it| it.get_pos() == pos)
    }

    pub fn mark_removed_entity(&mut self, id: EntityId) {
        if !self.remove_entity_queue.contains(&id) {
            self.remove_entity_queue.push(id);
        }
    }

    pub fn find_player_at_pos_mut(&mut self, pos: Pos) -> Option<&mut Player> {
        self.entities.values_mut().find_map(|it| {
            let player = it.as_any_mut().downcast_mut::<Player>()?;
            if player.next_pos() == pos {
                Some(player)
            } else {
                None
            }
        })
    }
}

pub trait Ticks {
    fn from_ticks(ticks: u64) -> Duration;
}

impl Ticks for Duration {
    fn from_ticks(ticks: u64) -> Duration {
        Duration::from_secs(ticks * 20)
    }
}