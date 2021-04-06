use super::position::*;
use super::*;

pub struct Snake {
    pub sign: char,
    pub head: Position,
    pub tail: Vec<Position>,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            sign: PLAYER_SIGN,
            head: Position { x: 1, y: 0 },
            tail: vec![Position { x: 0, y: 0 }],
        }
    }
    pub fn update(&mut self, dir: &Direction) {
        self.head.move_to_dir(dir);
        for tail_part in self.tail.iter_mut() {
            tail_part.move_to_dir(dir);
        }
    }
}
