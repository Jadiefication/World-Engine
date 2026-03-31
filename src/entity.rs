use std::any::Any;
use crate::world::World;

pub type EntityId = i32;

pub trait Entity {
    fn pos(&self) -> (usize, usize);
    fn set_pos(&mut self, pos: (usize, usize), world: &mut World) -> (usize, usize);
    fn process(&mut self, world: &mut World);
    fn get_id(&self) -> EntityId;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}