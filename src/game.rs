use super::board::*;
use super::position::*;
use super::snake::*;
use std::io;

pub fn run() {
    let mut board = Board::new();
    let mut snake = Snake::new();
    loop {
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

        snake.next_move(&direction);
        //update board
        board.update(&mut snake);
    }
}
