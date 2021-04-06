use std::io;
use super::*;
use super::position::*;

pub struct Snake {
    pub sign: char,
    pub position: Vec<Position>,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            sign: PLAYER_SIGN,
            position: vec![Position { x: 0, y: 0 }],
        }
    }
    pub fn reposition(&mut self) {
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