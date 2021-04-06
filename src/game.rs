use super::board::*;

pub fn run() {
    let mut board = Board::new();
    loop {
        board.reposition();
        board.draw();
        board.snake.reposition();
    }
}
