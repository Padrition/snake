use std::io;

pub struct Board {
    board : Vec<Vec<char>>,
    player : Player,
}
struct Player {
    sign : char,
    len : u32,
    possition_x : usize,
    possition_y : usize,
}
impl Player{
    fn new() -> Player{
        Player{
            sign : '#',
            len : 1,
            possition_x : 0,
            possition_y : 0,
        }
    }
}
impl Board{
    pub fn new() -> Board{
        Board{
            board : vec![vec!['0';10];10],
            player : Player::new(),
            
        }
    }
    fn move_player(&mut self){
        let mut dest = String::new();
        
        io::stdin()
        .read_line(&mut dest)
        .expect("Error reading the input");
        
        match dest.trim(){
            "w" => {
                self.player.possition_y -= 1;
            },
            "s" => {
                self.player.possition_y += 1;
            },
            "a" => {
                self.player.possition_x -= 1;
            },
            "d" => {
                self.player.possition_x += 1;
            },
            &_ => panic!("Unresolved input")
        }
    }
    fn board_set(&mut self){
        for y in 0..self.board.len(){
            for x in 0..self.board.len(){
                self.board[y][x] = '0';
            }
        }
        self.board[self.player.possition_y][self.player.possition_x] = self.player.sign;
    }
    fn draw(&mut self){
        self.board_set();
        print!("{}[2J", 27 as char);
        for row in self.board.iter(){
            for cell in row.iter(){
                print!("{}", cell);
            }
            println!();
        }
        self.move_player();
    }
}
pub fn run(){
    let mut b = Board::new();
    loop{
        b.draw();
    }
}