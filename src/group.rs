use super::coord::Coord;

#[derive(Clone, Debug)]
pub struct Group {
    pub coordinates: Vec<Coord>
}

impl Group {
    pub fn new (coordinates: Vec<Coord>) -> Self {
        Group {
            coordinates,
        }
    }

    pub fn contains_coord(&self, coord: &Coord) -> bool {
        self.coordinates.contains(coord)
    }
}
