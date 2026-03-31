use crate::world::World;

pub type EntityId = i32;

pub trait Entity {
    fn pos(&self) -> (usize, usize);
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize);
    fn process(&mut self, world: &World, xy: (usize, usize));
    fn get_id(&self) -> EntityId;
}