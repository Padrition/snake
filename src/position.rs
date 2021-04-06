pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position{
    pub fn move_to_dir(&self, dir : Direction){
        match dir{
            Direction::Up =>{},
            Direction::Right =>{},
            Direction::Left =>{},
            Direction::Down =>{},
        }
    }
}

pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}