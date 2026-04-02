use std::any::Any;
use std::time::Duration;
use rand::Rng;
use crate::entity::{Entity, EntityId, Pos};
use crate::event::event::Event;
use crate::world::{Ticks, World};

pub struct Obstacle {
    pos: Pos,
    max_durability: f32,
    current_durability: f32,
    duration: Option<Duration>,
    id: EntityId
}

impl Obstacle {
    pub fn new() -> Obstacle {
        Obstacle {
            pos: (0, 0),
            max_durability: -1.0,
            current_durability: -1.0,
            duration: None,
            id: (rand::rng().next_u32() as i32) as EntityId
        }
    }

    pub fn from(
        pos: Pos,
        max_durability: f32,
        current_durability: f32,
        duration: Option<Duration>
    ) -> Obstacle {
        Obstacle {
            pos,
            max_durability,
            current_durability,
            duration,
            id: (rand::rng().next_u32() as i32) as EntityId
        }
    }
}

impl Entity for Obstacle {
    fn process(&mut self, world: &mut World, _event: &Event) {
        if self.duration.is_some() {
            if let Some(new_duration) = self.duration.unwrap()
                .checked_sub(Duration::from_ticks(1)) {
                self.duration = Option::from(new_duration);
            } else {
                world.mark_removed_entity(self.id)
            }
        }
        if self.current_durability == 0.0 && self.max_durability != -1.0 {
            world.mark_removed_entity(self.id)
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