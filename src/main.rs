mod game;

use game::*;

fn main() {
    println!("\tS N A K E");
    
    let b = Board::new();

    b.draw();
}
