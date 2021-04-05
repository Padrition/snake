use std::io;

const PLAYER_SIGN: char = '#';
const BOARD_SIGN: char = '.';
const BOARD_SIZE: usize = 10;

pub struct Board {
    board: Vec<Vec<char>>,
    player: Player,
}
struct Player {
    sign: char,
    position: Vec<Coords>,
}
struct Coords {
    x: usize,
    y: usize,
}
impl Player {
    fn new() -> Player {
        Player {
            sign: PLAYER_SIGN,
            position: vec![Coords { x: 0, y: 0 }],
        }
    }
    fn reposition(&mut self) {
        let mut dest = String::new();

        io::stdin()
            .read_line(&mut dest)
            .expect("Error reading the input");

        match dest.trim() {
            "w" => {
                self.position[0].y -= 1;
            }
            "s" => {
                self.position[0].y += 1;
            }
            "a" => {
                self.position[0].x -= 1;
            }
            "d" => {
                self.position[0].x += 1;
            }
            &_ => panic!("Unresolved input"),
        }
    }
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![BOARD_SIGN; BOARD_SIZE]; BOARD_SIZE],
            player: Player::new(),
        }
    }
    fn reset(&mut self) {
        for y in 0..self.board.len() {
            for x in 0..self.board.len() {
                self.board[y][x] = BOARD_SIGN;
            }
        }
        self.board[self.player.position[0].y][self.player.position[0].x] = self.player.sign;
    }
    fn draw(&mut self) {
        print!("{}[2J", 27 as char);
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }
}
pub fn run() {
    let mut board = Board::new();
    loop {
        board.reset();
        board.draw();
        board.player.reposition();
    }
}
