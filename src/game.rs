use super::apple::*;
use super::board::*;
use super::position::*;
use super::snake::*;
use std::io;

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

        if let Err(DirectionError::OppositeDirection) = snake.next_move(&direction) {
            eprintln!("YOU ARE NOT ALLOWED TO GO OPPOSITE DIRECTION!");
            continue;
        };

        if snake.head == apple.pos {
            apple.eaten();
            snake.grow();
        }

        if apple.eaten {
            apple.update_pos();
        }
    }
}
