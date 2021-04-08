use super::position::*;

pub enum SnakeError {
    Direction(DirectionError),
    Position(PositionError),
}
pub struct Snake {
    pub current_dir: Direction,
    pub head: Position,
    pub tail: Vec<Position>,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            current_dir: Direction::Right,
            head: Position { x: 1, y: 0 },
            tail: vec![Position { x: 0, y: 0 }],
        }
    }
    pub fn next_move(&mut self, dir: &Direction) -> Result<(), SnakeError> {
        //player can't go to the opposite direction
        if *dir == self.current_dir.opposite() {
            return Err(SnakeError::Direction(DirectionError::OppositeDirection));
        }
        if self.tail.iter().any(|tail| *tail == self.head) {
            return Err(SnakeError::Position(PositionError::CollidingPositions));
        }
        //increment every tail part to the next position
        for i in (0..self.tail.len()).rev() {
            if i == 0 {
                break;
            } else {
                self.tail[i] = self.tail[i - 1].clone();
            }
        }
        //set first tail part's position to head's position
        self.tail[0] = self.head.clone();
        //move a head into the direction
        self.head.move_to_dir(dir);

        self.current_dir = *dir;

        Ok(())
    }
    pub fn grow(&mut self) {
        let last_index = self.tail.len() - 1;
        self.tail.push(self.tail[last_index].clone());
    }
}
