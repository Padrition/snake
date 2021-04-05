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
    pub x: usize,
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
    fn reposition(&mut self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                self.board[y][x] = BOARD_SIGN;
            }
        }
        for coordinate in self.player.position.iter() {
            self.board[coordinate.y][coordinate.x] = self.player.sign;
        }
    }
    fn draw(&mut self) {
        print!("{}[2J", 27 as char);
        for row in self.board.iter() {
            for sign in row.iter() {
                print!("{}", sign);
            }
            println!();
        }
    }
}
pub fn run() {
    let mut board = Board::new();
    loop {
        board.reposition();
        board.draw();
        board.player.reposition();
    }
}
