use super::position::*;

pub struct Snake {
    pub head: Position,
    pub tail: Vec<Position>,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            head: Position { x: 1, y: 0 },
            tail: vec![Position { x: 0, y: 0 }],
        }
    }
    pub fn next_move(&mut self, dir: &Direction) {
        //set first tail part's position to head's position
        self.tail[0] = self.head.clone();
        //increment every tail part to the next position
        for i in (1..self.tail.len()).rev() {
            if i == 0 {
                break;
            } else {
                self.tail[i] = self.tail[i - 1].clone();
            }
        }
        //move a head into the direction
        self.head.move_to_dir(dir);
    }
}
