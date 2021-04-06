use super::position::*;
use super::*;

pub struct Snake {
    pub sign: char,
    pub position: Vec<Position>,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            sign: PLAYER_SIGN,
            position: vec![Position { x: 0, y: 0 }],
        }
    }
}
