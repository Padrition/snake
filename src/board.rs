use super::apple::*;
use super::snake::*;
use super::*;
use std::fmt;

#[derive(Clone)]
enum Cell {
    Empty,
    Tail,
    Head,
    Apple,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "{}", BOARD_SIGN),
            Cell::Tail => write!(f, "{}", TAIL_SIGN),
            Cell::Head => write!(f, "{}", HEAD_SIGN),
            Cell::Apple => write!(f, "{}", APPLE_SIGN),
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
    pub fn update(&mut self, snake: &mut Snake, apple: &mut Apple) {
        //reset the board to empty
        self.board.fill(vec![Cell::Empty; BOARD_SIZE]);
        //take a position of snake's head and set cell for Head in the corresponding position in the board
        self.board[snake.head.y][snake.head.x] = Cell::Head;
        //iterate over snake's tail and set cell with a corresponding position to Tail
        for tail_part in snake.tail.iter() {
            self.board[tail_part.y][tail_part.x] = Cell::Tail;
        }
        //set Cell to Apple to corresponding apple position
        self.board[apple.pos.y][apple.pos.x] = Cell::Apple;
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
