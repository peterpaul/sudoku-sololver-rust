//! # Sudoku
//!
//! A sudoku solver in Rust.

mod repeater;
mod cell;
mod coord;
mod group;
mod cell_container;

use cell::Cell;
use coord::Coord;
use group::Group;
use cell_container::CellContainer;
use cell_container::pretty_print;

#[derive(Clone)]
pub struct Board {
    cells: CellContainer,
    groups: Vec<Group>
}

impl Board {
    /// Create a square sudoku puzzle.
    ///
    /// `base` is the square root of the width/height of the puzzle.
    /// So for a regular 9x9 puzzle, use 3.
    ///
    /// All rows and columns of the sudoku board are added as group.
    /// And smaller blocks of `block_width` * `block_height`.
    fn new(block_width: usize, block_height: usize) -> Self {
        let group_size = block_width * block_height;
        let mut groups = Vec::new();
        for x in 0..group_size {
            let mut coords = Vec::new();
            for y in 0..group_size {
                coords.push(Coord::new(x, y));
            }
            groups.push(Group::new(coords));
        }
        for y in 0..group_size {
            let mut coords = Vec::new();
            for x in 0..group_size {
                coords.push(Coord::new(x, y));
            }
            groups.push(Group::new(coords));
        }
        for xx in 0..(group_size/block_width) {
            for yy in 0..(group_size/block_height) {
                let mut coords = Vec::new();
                for x in 0..block_width {
                    for y in 0..block_height {
                        coords.push(Coord::new(xx * block_width + x, yy * block_height + y));
                    }
                }
                groups.push(Group::new(coords));
            }
        }
        Board {
            cells: CellContainer::new(block_width, block_height),
            groups: groups
        }
    }

    fn new_nrc() -> Self {
        let board = Board::new(3, 3);
        let mut groups = board.groups;
        for xx in 0..2 {
            for yy in 0..2 {
                let mut coords = Vec::new();
                for x in 0..3 {
                    for y in 0..3 {
                        coords.push(Coord::new(xx * 4 + 1 + x, yy * 4 + 1 + y));
                    }
                }
                groups.push(Group::new(coords));
            }
        }
        Board {
            cells: board.cells,
            groups: groups,
        }
    }

    pub fn from_string(s: &str) -> Self {
        let mut board = Board::new(3, 3);
        for l in s.lines() {
            let numbers: Vec<_> = l.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
            assert!(numbers.len() == 3);
            board.prefill_value(&Coord::new(numbers[0] - 1, numbers[1] - 1),
                                numbers[2] - 1);
        }
        board
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        self.cells.get_cell(coord)
    }

    fn group_size(&self) -> usize {
        self.cells.group_size()
    }

    pub fn pretty_print(&self) {
        println!("{}", pretty_print(&self.cells));
    }

    fn set_value(&mut self, coord: &Coord, value: usize) {
        self.set_value_by(coord, value, |cell, value| { cell.set_value(value) })
    }

    fn prefill_value(&mut self, coord: &Coord, value: usize) {
        self.set_value_by(coord, value, |cell, value| { cell.prefill_value(value) })
    }

    fn set_value_by(&mut self, coord: &Coord, value: usize, setter: fn(&mut Cell, usize)) {
        let cells = &mut self.cells;
        let groups: Vec<&Group> = self.groups
            .iter()
            .filter(|g| { g.contains_coord(coord) })
            .collect();
        for g in groups {
            for cur in &g.coordinates {
                let cell = cells.get_mut_cell(&cur);
                if cur == coord {
                    setter(cell, value);
                } else {
                    cell.strike_through(value);
                }
            }
        }
    }

    fn discover_new_values(&mut self) {
        let coords_to_update: Vec<Coord>;
        {
            coords_to_update = self.cells.get_cell_coords_to_update();
        }
        let discovered_new_values = !coords_to_update.is_empty();
        for coord in coords_to_update {
            let cell;
            {
                cell = self.get_cell(&coord);
            }
            match cell.get_value() {
                Some(v) => self.set_value(&coord, v),
                None => panic!("I expect some value here, because of cells.get_cell_coords_to_update()"),
            };
        }
        if discovered_new_values {
            self.discover_new_values();
        }
    }

    pub fn is_solved(&self) -> bool {
        let mut is_solved = true;
        for y in 0..self.group_size() {
            for x in 0..self.group_size() {
                is_solved &= self.get_cell(&Coord::new(x, y)).is_set
            }
        }
        is_solved
    }

    pub fn solve(&self) -> Vec<Self> {
        let mut puzzle = self.clone();
        puzzle.discover_new_values();
        let pivot = puzzle.find_pivot_coord();
        if puzzle.is_solved() {
            let mut result = Vec::new();
            result.push(puzzle);
            result
        } else {
            if let Some(p) = pivot {
                let pivot_cell = puzzle.get_cell(&p);
                (0..self.group_size()).into_iter().flat_map(|i| {
                    let i = i as usize;
                    if pivot_cell.possible_values[i] {
                        let mut subpuzzle = puzzle.clone();
                        subpuzzle.cells.get_mut_cell(&p).set_value(i);
                        subpuzzle.solve()
                    } else {
                        Vec::new()
                    }
                })
                    .take(1000)
                    .collect()
            } else {
                Vec::new()
            }
        }
    }

    fn find_pivot_coord(&self) -> Option<Coord> {
        let mut open_cells: Vec<(Coord, usize)> = Vec::new();
        for y in 0..self.group_size() {
            for x in 0..self.group_size() {
                let coord = Coord::new(x, y);
                let cell = self.get_cell(&coord);
                if !cell.is_set {
                    open_cells.push((coord, cell.possibilities()));
                }
            }
        }

        let mut result: Option<(Coord, usize)> = None;

        for c in open_cells {
            match result {
                Some(v) => {
                    result = if c.1 < v.1 {
                        Some(c)
                    } else {
                        Some(v)
                    }
                },
                None => result = Some(c),
            }
        }
        match result {
            Some((coord, _)) => {
                Some(coord)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strike_through_forward_works() {
        let mut cell = Cell::new(9);
        for i in 0..8 {
            assert_eq!(cell.get_value(), None);
            cell.strike_through(i);
        }
        assert_eq!(cell.get_value(), Some(8));
    }

    #[test]
    fn strike_through_backward_works() {
        let mut cell = Cell::new(9);
        for i in (1..9).rev() {
            assert_eq!(cell.get_value(), None);
            cell.strike_through(i);
        }
        assert_eq!(cell.get_value(), Some(0));
    }

    #[test]
    #[should_panic]
    fn strike_through_set_value() {
        let mut cell = Cell::new(9);
        cell.set_value(4);
        cell.strike_through(4);
    }

    #[test]
    fn cell_set_value_works() {
        let mut cell = Cell::new(9);
        cell.set_value(4);
        assert_eq!(cell.get_value(), Some(4));
    }

    #[test]
    fn test_board() {
        let mut board = Board::new(3, 3);
        board.set_value(&Coord::new(3, 3), 3);
        assert_eq!(board.get_cell(&Coord::new(3, 3)).get_value(), Some(3));
    }

    #[test]
    fn solve_nrc_puzzle() {
        let mut board = Board::new_nrc();
        board.prefill_value(&Coord::new(3, 1), 3);
        board.prefill_value(&Coord::new(6, 1), 4);
        board.prefill_value(&Coord::new(3, 2), 1);
        board.prefill_value(&Coord::new(8, 2), 7);
        board.prefill_value(&Coord::new(1, 3), 8);
        board.prefill_value(&Coord::new(2, 3), 0);
        board.prefill_value(&Coord::new(3, 3), 4);
        board.prefill_value(&Coord::new(6, 3), 6);
        board.prefill_value(&Coord::new(1, 4), 1);
        board.prefill_value(&Coord::new(2, 4), 6);
        board.prefill_value(&Coord::new(3, 4), 7);
        board.prefill_value(&Coord::new(4, 4), 5);
        board.prefill_value(&Coord::new(0, 5), 2);
        board.prefill_value(&Coord::new(3, 5), 0);
        board.prefill_value(&Coord::new(4, 5), 3);
        board.prefill_value(&Coord::new(6, 5), 8);
        board.prefill_value(&Coord::new(5, 6), 2);
        board.prefill_value(&Coord::new(0, 7), 6);
        board.prefill_value(&Coord::new(2, 7), 2);
        board.prefill_value(&Coord::new(4, 7), 4);
        board.prefill_value(&Coord::new(6, 7), 7);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_puzzle() {
        let mut board = Board::new(3, 3);
        board.prefill_value(&Coord::new(0, 0), 4);
        board.prefill_value(&Coord::new(1, 0), 7);
        board.prefill_value(&Coord::new(2, 0), 3);
        board.prefill_value(&Coord::new(3, 0), 8);
        board.prefill_value(&Coord::new(4, 0), 1);
        board.prefill_value(&Coord::new(8, 0), 2);
        board.prefill_value(&Coord::new(1, 1), 6);
        board.prefill_value(&Coord::new(3, 1), 0);
        board.prefill_value(&Coord::new(4, 1), 5);
        board.prefill_value(&Coord::new(6, 1), 4);
        board.prefill_value(&Coord::new(7, 1), 3);
        board.prefill_value(&Coord::new(1, 2), 5);
        board.prefill_value(&Coord::new(3, 2), 4);
        board.prefill_value(&Coord::new(5, 2), 3);
        board.prefill_value(&Coord::new(1, 3), 1);
        board.prefill_value(&Coord::new(2, 3), 8);
        board.prefill_value(&Coord::new(4, 3), 6);
        board.prefill_value(&Coord::new(6, 3), 5);
        board.prefill_value(&Coord::new(7, 3), 4);
        board.prefill_value(&Coord::new(0, 4), 0);
        board.prefill_value(&Coord::new(3, 4), 1);
        board.prefill_value(&Coord::new(5, 4), 4);
        board.prefill_value(&Coord::new(0, 5), 6);
        board.prefill_value(&Coord::new(1, 5), 3);
        board.prefill_value(&Coord::new(2, 5), 4);
        board.prefill_value(&Coord::new(6, 5), 8);
        board.prefill_value(&Coord::new(2, 6), 6);
        board.prefill_value(&Coord::new(3, 6), 7);
        board.prefill_value(&Coord::new(6, 6), 2);
        board.prefill_value(&Coord::new(7, 6), 0);
        board.prefill_value(&Coord::new(2, 7), 2);
        board.prefill_value(&Coord::new(3, 7), 6);
        board.prefill_value(&Coord::new(4, 7), 0);
        board.prefill_value(&Coord::new(7, 7), 7);
        board.prefill_value(&Coord::new(8, 7), 4);
        board.prefill_value(&Coord::new(0, 8), 5);
        board.prefill_value(&Coord::new(2, 8), 7);
        board.prefill_value(&Coord::new(4, 8), 4);
        board.prefill_value(&Coord::new(5, 8), 1);
        board.prefill_value(&Coord::new(7, 8), 8);
        board.prefill_value(&Coord::new(8, 8), 6);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_2_by_3_puzzle() {
        let mut board = Board::new(2, 3);
        board.prefill_value(&Coord::new(0, 0), 0);
        board.prefill_value(&Coord::new(1, 0), 1);
        board.prefill_value(&Coord::new(2, 0), 2);
        board.prefill_value(&Coord::new(3, 0), 3);
        board.prefill_value(&Coord::new(4, 0), 4);
        board.prefill_value(&Coord::new(5, 0), 5);
        board.prefill_value(&Coord::new(0, 1), 2);
        board.prefill_value(&Coord::new(1, 1), 3);
        board.prefill_value(&Coord::new(2, 1), 4);
        board.prefill_value(&Coord::new(3, 1), 5);
        board.prefill_value(&Coord::new(4, 1), 0);
        board.prefill_value(&Coord::new(5, 1), 1);
        board.prefill_value(&Coord::new(0, 2), 4);
        board.prefill_value(&Coord::new(1, 2), 5);
        board.prefill_value(&Coord::new(2, 2), 0);
        board.prefill_value(&Coord::new(3, 2), 1);
        board.prefill_value(&Coord::new(4, 2), 2);
        board.prefill_value(&Coord::new(5, 2), 3);
        board.prefill_value(&Coord::new(0, 3), 1);
        board.prefill_value(&Coord::new(1, 3), 0);
        board.prefill_value(&Coord::new(2, 3), 3);
        board.prefill_value(&Coord::new(3, 3), 2);
        board.prefill_value(&Coord::new(4, 3), 5);
        board.prefill_value(&Coord::new(5, 3), 4);
        board.prefill_value(&Coord::new(0, 4), 3);
        board.prefill_value(&Coord::new(1, 4), 2);
        board.prefill_value(&Coord::new(0, 5), 5);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_2_by_1_puzzle() {
        let mut board = Board::new(2, 1);
        board.prefill_value(&Coord::new(0, 0), 0);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }
}
