use std::cell::RefCell;
use crate::entity::Entity;

pub struct World {
    entities: Vec<Vec<RefCell<Box<dyn Entity>>>>,
    size: (u32, u32),
    entity_queue: Vec<(Box<dyn Entity>, (usize, usize))>
}

impl World {
    fn new() -> World {
        World { entities: vec![], size: (256, 256), entity_queue: vec![] }
    }

    fn from(entities: Vec<Vec<RefCell<Box<dyn Entity>>>>, size: (u32, u32)) -> World {
        World { entities, size, entity_queue: vec![] }
    }

    fn process(&mut self) {
        for x in 0..self.entities.len() {
            for y in 0..self.entities[x].len() {
                self.entities[x][y].borrow_mut().process(self, (x, y));
            }
        }
        for (entity, pos) in self.entity_queue.drain(..) {
            self.entities[pos.0][pos.1] = RefCell::from(entity)
        }
    }

    fn spawn_entity(&mut self, entity: Box<dyn Entity>, pos: (usize, usize)) {
        self.entity_queue.push((entity, pos))
    }
}

macro_rules! world {
    ($x_width:expr => $y_width:expr, $( $x:expr ),*) => {
        World {
            entities: vec![$($x),*],
            size: ($x_width, $y_width),
        }
    };
}