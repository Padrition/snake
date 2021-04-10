use super::*;

pub enum PositionError {
    CollidingPositions,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn move_to_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    self.y = BOARD_SIZE - 1;
                } else {
                    self.y -= 1;
                }
            }
            Direction::Right => {
                if self.x == BOARD_SIZE - 1 {
                    self.x = 0;
                } else {
                    self.x += 1;
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    self.x = BOARD_SIZE - 1;
                } else {
                    self.x -= 1;
                }
            }
            Direction::Down => {
                if self.y == BOARD_SIZE - 1 {
                    self.y = 0;
                } else {
                    self.y += 1;
                }
            }
        }
    }
}

pub enum DirectionError {
    UnknownDirection,
    OppositeDirection,
}
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}
impl Direction {
    pub fn set(dir: &str) -> Result<Direction, DirectionError> {
        match dir.trim() {
            "w" => Ok(Direction::Up),
            "d" => Ok(Direction::Right),
            "a" => Ok(Direction::Left),
            "s" => Ok(Direction::Down),
            _ => Err(DirectionError::UnknownDirection),
        }
    }
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }
}
