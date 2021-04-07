use super::position::*;
use super::*;
use rand::prelude::*;

pub struct Apple {
    pub pos: Position,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            pos: Position {
                x: thread_rng().gen_range(0..BOARD_SIZE),
                y: thread_rng().gen_range(0..BOARD_SIZE),
            },
        }
    }
}
