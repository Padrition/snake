mod game;
mod snake;
mod position;
mod board;

pub const PLAYER_SIGN: char = '#';
pub const BOARD_SIGN: char = '.';
pub const BOARD_SIZE: usize = 10;

fn main() {
    println!("\tS N A K E");

    game::run();
}
