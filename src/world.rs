use std::cell::RefCell;
use crate::entity::Entity;

pub struct World {
    entities: Vec<Vec<RefCell<Box<dyn Entity>>>>,
    size: (u32, u32)
}

impl World {
    fn process(&mut self) {
        for x in 0..self.entities.len() {
            for y in 0..self.entities[x].len() {
                self.entities[x][y].borrow_mut().process(self, (x, y));
            }
        }
    }
}