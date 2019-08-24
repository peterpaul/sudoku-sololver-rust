use super::cell::Cell;
use super::repeater::Repeater;
use super::coord::Coord;

#[derive(Clone)]
pub struct CellContainer {
    group_size: usize,
    cells: Box<[Cell]>,
}

impl CellContainer {
    pub fn new(group_size: usize) -> Self {
        let cells: Vec<Cell> = Repeater::new(Box::new(move || { Cell::new(group_size) }))
            .take(group_size * group_size)
            .collect();
        CellContainer {
            group_size,
            cells: cells.into_boxed_slice(),
        }
    }

    pub fn group_size(&self) -> usize {
        self.group_size
    }

    fn index_of(&self, coord: &Coord) -> usize {
        coord.y * self.group_size() + coord.x
    }

    pub fn get_cell(&self, coord: &Coord) -> &Cell {
        &self.cells[self.index_of(coord)]
    }

    pub fn get_mut_cell(&mut self, coord: &Coord) -> &mut Cell {
        &mut self.cells[self.index_of(coord)]
    }

    pub fn get_cell_coords_to_update(&self) -> Vec<Coord> {
        let mut cell_coords_to_update = Vec::new();
        let group_size = self.group_size();
        for y in 0..group_size {
            for x in 0..group_size {
                let coord = Coord::new(x, y);
                let cell = self.get_cell(&coord);
                if !cell.is_set && cell.get_value().is_some() {
                    cell_coords_to_update.push(coord);
                }
            }
        }
        cell_coords_to_update
    }
}
