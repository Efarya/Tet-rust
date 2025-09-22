pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub const SHAPE: char = '█';

pub type Cell = Option<char>; // None = vide, Some('█') = bloc

#[derive(Debug)]
pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![vec![None; WIDTH]; HEIGHT],
        }
    }
}
