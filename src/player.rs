use std::any::Any;
use rand::Rng;
use crate::action::Action;
use crate::action::Action::Nothing;
use crate::entity::{Entity, EntityId, Pos};
use crate::obstacle::Obstacle;
use crate::world::{World, WorldContext};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Player {
    pos: Pos,
    id: EntityId,
    direction: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (0, 0),
            id: rand::rng().next_u32() as EntityId,
            direction: Direction::Up,
        }
    }

    pub fn from(
        pos: Pos,
        direction: Direction) -> Player {
        Player {
            pos,
            id: rand::rng().next_u32() as EntityId,
            direction,
        }
    }

    pub fn next_pos(&self) -> Pos {
        match self.direction {
            Direction::Up => (self.pos.0, self.pos.1 + 1 ),
            Direction::Down => (self.pos.0, self.pos.1 - 1 ),
            Direction::Left => (self.pos.0 - 1, self.pos.1 ),
            Direction::Right => (self.pos.0 + 1, self.pos.1 )
        }
    }

    fn check_for_collision(&self, world: &World) -> bool {
        let next_pos = self.next_pos();
        if let Some(entity) = world.find_by_pos(next_pos) {
            if let Some(_) = entity.as_any().downcast_ref::<Obstacle>() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Entity for Player {

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

    fn update_pos(&mut self, world: &World) {
        let new_pos = self.next_pos();
        world.find_by_pos(new_pos);
        if new_pos.0 <= world.size.0 && new_pos.1 <= world.size.1
            && !self.check_for_collision(world)
        {
            self.pos = new_pos;
        }
    }

    fn get_pos(&self) -> Pos {
        self.pos
    }
}