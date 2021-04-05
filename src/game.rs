use std::io;

const PLAYER_SIGN: char = '#';
const BOARD_SIGN: char = '.';
pub struct Board {
    board: Vec<Vec<char>>,
    player: Player,
}
struct Player {
    sign: char,
    position: Vec<Cords>,
}
struct Cords {
    x: usize,
    y: usize,
}
impl Player {
    fn new() -> Player {
        Player {
            sign: PLAYER_SIGN,
            position: vec![Cords { x: 0, y: 0 }],
        }
    }
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![BOARD_SIGN; 10]; 10],
            player: Player::new(),
        }
    }
    fn move_player(&mut self) {
        let mut dest = String::new();

        io::stdin()
            .read_line(&mut dest)
            .expect("Error reading the input");

        match dest.trim() {
            "w" => {
                self.player.position[0].y -= 1;
            }
            "s" => {
                self.player.position[0].y += 1;
            }
            "a" => {
                self.player.position[0].x -= 1;
            }
            "d" => {
                self.player.position[0].x += 1;
            }
            &_ => panic!("Unresolved input"),
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
        self.move_player();
    }
}
pub fn run() {
    let mut board = Board::new();
    loop {
        board.reset();
        board.draw();
    }
}
