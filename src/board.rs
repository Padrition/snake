use super::*;
use super::snake::*;

pub struct Board {
    board: Vec<Vec<char>>,
    pub snake: Snake,
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![BOARD_SIGN; BOARD_SIZE]; BOARD_SIZE],
            snake: Snake::new(),
        }
    }
    pub fn reposition(&mut self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                self.board[y][x] = BOARD_SIGN;
            }
        }
        for coordinate in self.snake.position.iter() {
            self.board[coordinate.y][coordinate.x] = self.snake.sign;
        }
    }
    pub fn draw(&mut self) {
        print!("{}[2J", 27 as char);//clears the terminal
        for row in self.board.iter() {
            for sign in row.iter() {
                print!("{}", sign);
            }
            println!();
        }
    }
}