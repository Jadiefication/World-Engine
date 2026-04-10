use std::any::Any;
use std::time::Duration;
use rand::Rng;
use crate::action::Action;
use crate::action::Action::Nothing;
use crate::entity::{Entity, EntityId, Pos};
use crate::world::{Ticks, World, WorldContext};

pub struct Obstacle {
    pos: Pos,
    id: EntityId
}

impl Obstacle {
    pub fn new() -> Obstacle {
        Obstacle {
            pos: (0, 0),
            id: rand::rng().next_u32() as EntityId
        }
    }

    pub fn from(
        pos: Pos) -> Obstacle {
        Obstacle {
            pos,
            id: rand::rng().next_u32() as EntityId
        }
    }
}

impl Entity for Obstacle {
    fn process(&mut self, _world: &mut WorldContext) -> Action {
        Nothing
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

    fn update_pos(&mut self, _world: &World) {
        return;
    }

    fn get_pos(&self) -> Pos {
        todo!()
    }
}