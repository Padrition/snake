use super::snake::*;
use super::*;

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
    pub fn draw(&mut self) {
        print!("{}[2J", 27 as char); //clears the terminal
        for row in self.board.iter() {
            for sign in row.iter() {
                print!("{}", sign);
            }
            println!();
        }
    }
}
