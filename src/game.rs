use super::apple::*;
use super::board::*;
use super::position::*;
use super::snake::*;
use std::io;
use std::process;

pub fn run() {
    let mut board = Board::new();
    let mut snake = Snake::new();
    let mut apple = Apple::new();
    loop {
        //update board
        board.update(&mut snake, &mut apple);
        //draw a board
        board.draw();
        //ask for direction
        let mut dir_str = String::new();
        io::stdin()
            .read_line(&mut dir_str)
            .expect("Error reading the input");

        let direction = match Direction::set(&dir_str) {
            Ok(dir) => dir,
            Err(_) => panic!("UnknownDirection"),
        };

        match snake.next_move(&direction) {
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

        if snake.head == apple.pos {
            apple.eaten();
            snake.grow();
        }

        apple.update_pos();
    }
}
