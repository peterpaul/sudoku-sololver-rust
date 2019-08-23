use super::cell::Cell;
use super::repeater::Repeater;
use super::coord::Coord;

#[derive(Clone)]
pub struct CellContainer {
    block_width: usize,
    block_height: usize,
    cells: Box<[Cell]>,
}

impl CellContainer {
    pub fn new(block_width: usize, block_height: usize) -> Self {
        let group_size = block_width * block_height;
        let cells: Vec<Cell> = Repeater::new(Box::new(move || { Cell::new(group_size) }))
            .take(group_size * group_size)
            .collect();
        CellContainer {
            block_width,
            block_height,
            cells: cells.into_boxed_slice(),
        }
    }

    pub fn group_size(&self) -> usize {
        self.block_width * self.block_height
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

fn pretty_print_separator_row(cell_container: &CellContainer, result: &mut String) {
    let group_size = cell_container.group_size();
    for _xx in 0..(group_size/cell_container.block_width) {
        for x in 0..cell_container.block_width {
            if x == 0 {
                result.push_str("+---");
            } else {
                result.push_str("----");
            }
        }
    }
    result.push_str("+\n");
}

fn pretty_print_empty_row(cell_container: &CellContainer, result: &mut String) {
    let group_size = cell_container.group_size();
    for _xx in 0..(group_size/cell_container.block_width) {
        for x in 0..cell_container.block_width {
            if x == 0 {
                result.push_str("|   ");
            } else {
                result.push_str("    ");
            }
        }
    }
    result.push_str("|\n");
}

pub fn pretty_print(cell_container: &CellContainer) -> String {
    let group_size = cell_container.group_size();
    let mut result = String::new();
    for yy in 0..(group_size/cell_container.block_height) {
        for y in 0..cell_container.block_height {
            if y == 0 {
                pretty_print_separator_row(cell_container, &mut result);
            } else {
                pretty_print_empty_row(cell_container, &mut result);
            }
            for xx in 0..(group_size/cell_container.block_width) {
                for x in 0..cell_container.block_width {
                    let coord = Coord::new(
                        xx * cell_container.block_width + x,
                        yy * cell_container.block_height + y,
                    );
                    let v = match cell_container.get_cell(&coord).get_value() {
                        Some(v) => format!("{}", v + 1),
                        None => String::from(" "),
                    };
                    if x == 0 {
                        result.push_str(&format!("| {} ", v));
                    } else {
                        result.push_str(&format!("  {} ", v));
                    }
                }
            }
            result.push_str("|\n");
        }
    }
    pretty_print_separator_row(cell_container, &mut result);
    result
}
