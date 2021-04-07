use super::*;
use super::position::*;
use rand::prelude::*;

struct Apple {
    apple: Position,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            apple: Position {
                x: thread_rng().gen_range(0..BOARD_SIZE),
                y: thread_rng().gen_range(0..BOARD_SIZE),
            },
        }
    }
}
