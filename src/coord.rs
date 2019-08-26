#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Coord {
            x,
            y,
        }
    }
}
