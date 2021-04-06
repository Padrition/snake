use super::*;
use std::fmt;

#[derive(Clone)]
enum Cell{
    Empty,
    Player,
    Apple
}
impl fmt::Display for Cell{
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Cell::Empty => write!(f, "."),
            Cell::Player => write!(f, "#"),
            Cell::Apple => write!(f, "@"),
        }
    }
}

pub struct Board {
    board: Vec<Vec<Cell>>,
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
    pub fn draw(&mut self) {
        print!("{}[2J", 27 as char); //clears the terminal
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }
}
