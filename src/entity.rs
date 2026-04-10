use std::any::Any;
use crate::world::{World, WorldContext};
use crate::action::Action;

pub type EntityId = i32;
pub type Pos = (usize, usize);

pub trait Entity {
    fn process(&mut self, world: &mut WorldContext) -> Action;
    fn get_id(&self) -> EntityId;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn update_pos(&mut self, world: &World);
    fn get_pos(&self) -> Pos;
}