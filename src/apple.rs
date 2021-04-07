use super::position::*;
use super::*;
use rand::prelude::*;

pub struct Apple {
    pub eaten : bool,
    pub pos: Position,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            eaten : false,
            pos: Position {
                x: thread_rng().gen_range(0..BOARD_SIZE),
                y: thread_rng().gen_range(0..BOARD_SIZE),
            },
        }
    }
    pub fn update_pos(&mut self){
        self.eaten = false;
        self.pos.x = thread_rng().gen_range(0..BOARD_SIZE);
        self.pos.y = thread_rng().gen_range(0..BOARD_SIZE);
    }
    pub fn eaten(&mut self){
        self.eaten = true;
    }
}
