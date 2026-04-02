use crate::player::Direction;

#[derive(PartialEq)]
pub enum Event {
    Exit,
    Movement(Direction),
    Eat
}