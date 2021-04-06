use super::*;
use super::snake::*;
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
    pub fn update(&mut self, snake : &mut Snake){
        //reset the board to empty 
        self.board.fill(vec![Cell::Empty; BOARD_SIZE]);
        //take a position of snake's head and set cell for Player in the corresponding position in the board 
        self.board[snake.head.y][snake.head.x] = Cell::Player;
        //iterate over snake's tail and set cell with a corresponding position to Player 
        for tail_part in snake.tail.iter(){
            self.board[tail_part.y][tail_part.x] = Cell::Player;
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
