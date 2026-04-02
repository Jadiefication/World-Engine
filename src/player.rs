use std::any::Any;
use crate::entity::{Entity, EntityId, Pos};
use crate::event::event::Event;
use crate::food::Food;
use crate::obstacle::Obstacle;
use crate::world::World;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Player {
    pos: Pos,
    food_level: f32,
    id: EntityId,
    direction: Direction
}

impl Player {
    pub fn next_pos(&self) -> Pos {
        match self.direction {
            Direction::Up => (self.pos.0, self.pos.1 + 1 ),
            Direction::Down => (self.pos.0, self.pos.1 - 1 ),
            Direction::Left => (self.pos.0 - 1, self.pos.1 ),
            Direction::Right => (self.pos.0 + 1, self.pos.1 )
        }
    }

    fn check_for_food(&self, world: &World) -> bool {
        let next_pos = self.next_pos();
        if let Some(entity) = world.find_by_pos(next_pos) {
            if let Some(_) = entity.as_any().downcast_ref::<Food>() {
                true
            } else {
                false
            }
        } else {
            false
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

    fn process(&mut self, world: &mut World, event: &Event) {
        todo!()
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