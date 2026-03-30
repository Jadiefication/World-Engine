use crate::world::World;

pub trait Entity {
    fn pos(&self) -> (usize, usize);
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize);
    fn process(&mut self, world: &World, xy: (usize, usize));
}