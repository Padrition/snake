use super::apple::*;
use super::board::*;
use super::position::*;
use super::snake::*;
use std::io;
use std::process;
pub struct Game {
    board: Board,
    snake: Snake,
    apple: Apple,
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            snake: Snake::new(),
            apple: Apple::new(),
        }
    }
    pub fn run(&mut self) {
        loop {
            //update board
            self.board.update(&mut self.snake, &mut self.apple);
            //draw a board
            self.board.draw();
            //ask for direction
            let mut dir_str = String::new();
            io::stdin()
                .read_line(&mut dir_str)
                .expect("Error reading the input");

            let direction = match Direction::set(&dir_str) {
                Ok(dir) => dir,
                Err(_) => panic!("UnknownDirection"),
            };

            match self.snake.next_move(&direction) {
                Ok(a) => a,
                Err(SnakeError::Direction(DirectionError::UnknownDirection)) => {
                    println!("UNKNOWN DIRECTION! (USE \'w\', \'a\', \'s\', \'d\')");
                    continue;
                }
                Err(SnakeError::Direction(DirectionError::OppositeDirection)) => {
                    println!("YOU ARE NOT ALLOWED TO MOVE IN THE OPPOSITE DIRECTION!");
                    continue;
                }
                Err(SnakeError::Position(PositionError::CollidingPositions)) => {
                    println!("GAME OVER!");
                    println!("you collided with yourself");
                    process::exit(0);
                }
            }

            if self.snake.head == self.apple.pos {
                self.apple.eaten();
                self.snake.grow();
            }

            self.apple.update_pos();

            self.apple_pos_check();
        }
    }
    fn apple_pos_check(&mut self) {
        while self.snake.head == self.apple.pos {
            self.apple.update_pos();
        }
        for i in 0..self.snake.tail.len() {
            while self.snake.tail[i] == self.apple.pos {
                self.apple.update_pos();
            }
        }
    }
}
