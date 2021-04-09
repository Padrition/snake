mod apple;
mod board;
mod game;
mod position;
mod snake;

pub const TAIL_SIGN: char = '#';
pub const HEAD_SIGN: char = 'O';
pub const APPLE_SIGN: char = '@';
pub const BOARD_SIGN: char = '.';
pub const BOARD_SIZE: usize = 10;

fn main() {
    println!("\tS N A K E");

    let mut game = game::Game::new();

    game.run();

}
