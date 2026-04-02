use std::any::Any;
use crate::event::event::Event;
use crate::world::World;

pub type EntityId = i32;
pub type Pos = (usize, usize);

pub trait Entity {
    fn process(&mut self, world: &mut World, event: &Event);
    fn get_id(&self) -> EntityId;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}