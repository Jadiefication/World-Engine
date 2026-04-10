use std::any::Any;
use rand::Rng;
use crate::entity::{Entity, EntityId, Pos};
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
    direction: Direction,
    health: f32
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: (0, 0),
            food_level: 0.0,
            id: (rand::rng().next_u32() as i32) as EntityId,
            direction: Direction::Up,
            health: 100.0
        }
    }

    pub fn from(
        pos: Pos,
        food_level: f32,
        direction: Direction,
        health: f32
    ) -> Player {
        Player {
            pos,
            food_level,
            id: (rand::rng().next_u32() as i32) as EntityId,
            direction,
            health
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

    fn update_pos(&mut self, world: &mut World) {
        let new_pos = self.next_pos();
        world.find_by_pos(new_pos);
        if new_pos.0 <= world.size.0 && new_pos.1 <= world.size.1
            && !self.check_for_collision(world)
        {
            self.pos = new_pos;
            world.update_pos(self.id, new_pos);
        }
    }

    pub fn eat(&mut self, eat_amount: f32) {
        self.food_level += eat_amount;
    }
}

impl Entity for Player {

    fn process(&mut self, world: &mut World) {
        self.food_level -= 1.0;
        if self.food_level.max(0.0) == 0.0 {
            self.food_level = 0.0;
            self.health -= 1.0;
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