pub struct Board {
    board : Vec<Vec<char>>,
    player : Player,
}
struct Player {
    len : u32,
    possition_x : u32,
    possition_y : u32,
}
impl Player{
    fn new() -> Player{
        Player{
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
    pub fn draw(&self){
        for (i, row) in self.board.iter().enumerate(){
            for (j, cell) in row.iter().enumerate() {
                if j == self.player.possition_x as usize && i == self.player.possition_y as usize{
                    print!("#");
                }else{
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}