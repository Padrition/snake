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
        self.board[self.player.possition_y][self.player.possition_x] = self.player.sign;
    }
    fn draw(&mut self){
        print!("{}[2J", 27 as char);
        self.move_player();
        for row in self.board.iter(){
            for cell in row.iter(){
                print!("{}", cell);
            }
            println!();
        }
    }
}
pub fn run(){
    let mut b = Board::new();
    b.draw();
}