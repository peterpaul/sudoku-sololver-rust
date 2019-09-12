//! # Sudoku
//!
//! A sudoku solver in Rust.

mod repeater;
mod cell;
mod coord;
mod group;
mod cell_container;
mod board;

use std::ops::Deref;

use board::Board;
use coord::Coord;
use group::Group;
use cell_container::CellContainer;

trait BoardPrinter {
    fn pretty_print(&self) -> String;
}

pub struct RectangularBoard {
    block_width: usize,
    block_height: usize,
    board: Board,
}

impl Deref for RectangularBoard {
    type Target = Board;

    fn deref(&self) -> &Board {
        &self.board
    }
}

impl RectangularBoard {
    /// Create a square sudoku puzzle.
    ///
    /// `base` is the square root of the width/height of the puzzle.
    /// So for a regular 9x9 puzzle, use 3.
    ///
    /// All rows and columns of the sudoku board are added as group.
    /// And smaller blocks of `block_width` * `block_height`.
    pub fn new(block_width: usize, block_height: usize) -> RectangularBoard {
        assert!(block_width > 0);
        assert!(block_height > 0);
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
        if block_width != 1 && block_height != 1 {
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
        }
        let board = Board::new(
            CellContainer::new(group_size),
            groups
        );
        RectangularBoard {
            block_width,
            block_height,
            board,
        }
    }

    pub fn new_nrc() -> Self {
        let board = RectangularBoard::new(3, 3);
        let cells = board.board.cells;
        let mut groups = board.board.groups;
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
        RectangularBoard {
            block_width: 3,
            block_height: 3,
            board: Board::new(
                cells,
                groups,
            ),
        }
    }

    pub fn from_string(s: &str) -> Self {
        let mut board = RectangularBoard::new(3, 3);
        for l in s.lines() {
            let numbers: Vec<_> = l.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
            assert!(numbers.len() == 3);
            board.board.prefill_value(&Coord::new(numbers[0] - 1, numbers[1] - 1),
                                      numbers[2] - 1);
        }
        board
    }

    pub fn pretty_print(&self) {
        println!("{}", BoardPrinter::pretty_print(self));
    }

    fn pretty_print_separator_row(&self, result: &mut String) {
        let group_size = self.cells.group_size();
        for _xx in 0..(group_size/self.block_width) {
            for x in 0..self.block_width {
                if x == 0 {
                    result.push_str("+---");
                } else {
                    result.push_str("----");
                }
            }
        }
        result.push_str("+\n");
    }

    fn pretty_print_empty_row(&self, result: &mut String) {
        let group_size = self.cells.group_size();
        for _xx in 0..(group_size/self.block_width) {
            for x in 0..self.block_width {
                if x == 0 {
                    result.push_str("|   ");
                } else {
                    result.push_str("    ");
                }
            }
        }
        result.push_str("|\n");
    }

    pub fn count_solutions(self) -> usize {
        self.board.count_solutions()
    }

    pub fn solve(&self) -> Vec<Self> {
        self.board.solve()
            .into_iter()
            .map(|s| {
                RectangularBoard {
                    block_width: self.block_width,
                    block_height: self.block_height,
                    board: s,
                }
            })
            .collect()
    }
}

impl BoardPrinter for RectangularBoard {
    fn pretty_print(&self) -> String {
        let group_size = self.cells.group_size();
        let mut result = String::new();
        for yy in 0..(group_size/self.block_height) {
            for y in 0..self.block_height {
                if y == 0 {
                    self.pretty_print_separator_row(&mut result);
                } else {
                    self.pretty_print_empty_row(&mut result);
                }
                for xx in 0..(group_size/self.block_width) {
                    for x in 0..self.block_width {
                        let coord = Coord::new(
                            xx * self.block_width + x,
                            yy * self.block_height + y,
                        );
                        let v = match self.cells.get_cell(&coord).get_value() {
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
        self.pretty_print_separator_row(&mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::cell::Cell;

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
        let mut board = RectangularBoard::new(3, 3);
        board.board.set_value(&Coord::new(3, 3), 3);
        assert_eq!(board.get_cell(&Coord::new(3, 3)).get_value(), Some(3));
    }

    #[test]
    fn solve_nrc_puzzle() {
        let mut board = RectangularBoard::new_nrc();
        board.board.prefill_value(&Coord::new(3, 1), 3);
        board.board.prefill_value(&Coord::new(6, 1), 4);
        board.board.prefill_value(&Coord::new(3, 2), 1);
        board.board.prefill_value(&Coord::new(8, 2), 7);
        board.board.prefill_value(&Coord::new(1, 3), 8);
        board.board.prefill_value(&Coord::new(2, 3), 0);
        board.board.prefill_value(&Coord::new(3, 3), 4);
        board.board.prefill_value(&Coord::new(6, 3), 6);
        board.board.prefill_value(&Coord::new(1, 4), 1);
        board.board.prefill_value(&Coord::new(2, 4), 6);
        board.board.prefill_value(&Coord::new(3, 4), 7);
        board.board.prefill_value(&Coord::new(4, 4), 5);
        board.board.prefill_value(&Coord::new(0, 5), 2);
        board.board.prefill_value(&Coord::new(3, 5), 0);
        board.board.prefill_value(&Coord::new(4, 5), 3);
        board.board.prefill_value(&Coord::new(6, 5), 8);
        board.board.prefill_value(&Coord::new(5, 6), 2);
        board.board.prefill_value(&Coord::new(0, 7), 6);
        board.board.prefill_value(&Coord::new(2, 7), 2);
        board.board.prefill_value(&Coord::new(4, 7), 4);
        board.board.prefill_value(&Coord::new(6, 7), 7);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_puzzle() {
        let mut board = RectangularBoard::new(3, 3);
        board.board.prefill_value(&Coord::new(0, 0), 4);
        board.board.prefill_value(&Coord::new(1, 0), 7);
        board.board.prefill_value(&Coord::new(2, 0), 3);
        board.board.prefill_value(&Coord::new(3, 0), 8);
        board.board.prefill_value(&Coord::new(4, 0), 1);
        board.board.prefill_value(&Coord::new(8, 0), 2);
        board.board.prefill_value(&Coord::new(1, 1), 6);
        board.board.prefill_value(&Coord::new(3, 1), 0);
        board.board.prefill_value(&Coord::new(4, 1), 5);
        board.board.prefill_value(&Coord::new(6, 1), 4);
        board.board.prefill_value(&Coord::new(7, 1), 3);
        board.board.prefill_value(&Coord::new(1, 2), 5);
        board.board.prefill_value(&Coord::new(3, 2), 4);
        board.board.prefill_value(&Coord::new(5, 2), 3);
        board.board.prefill_value(&Coord::new(1, 3), 1);
        board.board.prefill_value(&Coord::new(2, 3), 8);
        board.board.prefill_value(&Coord::new(4, 3), 6);
        board.board.prefill_value(&Coord::new(6, 3), 5);
        board.board.prefill_value(&Coord::new(7, 3), 4);
        board.board.prefill_value(&Coord::new(0, 4), 0);
        board.board.prefill_value(&Coord::new(3, 4), 1);
        board.board.prefill_value(&Coord::new(5, 4), 4);
        board.board.prefill_value(&Coord::new(0, 5), 6);
        board.board.prefill_value(&Coord::new(1, 5), 3);
        board.board.prefill_value(&Coord::new(2, 5), 4);
        board.board.prefill_value(&Coord::new(6, 5), 8);
        board.board.prefill_value(&Coord::new(2, 6), 6);
        board.board.prefill_value(&Coord::new(3, 6), 7);
        board.board.prefill_value(&Coord::new(6, 6), 2);
        board.board.prefill_value(&Coord::new(7, 6), 0);
        board.board.prefill_value(&Coord::new(2, 7), 2);
        board.board.prefill_value(&Coord::new(3, 7), 6);
        board.board.prefill_value(&Coord::new(4, 7), 0);
        board.board.prefill_value(&Coord::new(7, 7), 7);
        board.board.prefill_value(&Coord::new(8, 7), 4);
        board.board.prefill_value(&Coord::new(0, 8), 5);
        board.board.prefill_value(&Coord::new(2, 8), 7);
        board.board.prefill_value(&Coord::new(4, 8), 4);
        board.board.prefill_value(&Coord::new(5, 8), 1);
        board.board.prefill_value(&Coord::new(7, 8), 8);
        board.board.prefill_value(&Coord::new(8, 8), 6);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_2_by_3_puzzle() {
        let mut board = RectangularBoard::new(2, 3);
        board.board.prefill_value(&Coord::new(0, 0), 0);
        board.board.prefill_value(&Coord::new(1, 0), 1);
        board.board.prefill_value(&Coord::new(2, 0), 2);
        board.board.prefill_value(&Coord::new(3, 0), 3);
        board.board.prefill_value(&Coord::new(4, 0), 4);
        board.board.prefill_value(&Coord::new(5, 0), 5);
        board.board.prefill_value(&Coord::new(0, 1), 2);
        board.board.prefill_value(&Coord::new(1, 1), 3);
        board.board.prefill_value(&Coord::new(2, 1), 4);
        board.board.prefill_value(&Coord::new(3, 1), 5);
        board.board.prefill_value(&Coord::new(4, 1), 0);
        board.board.prefill_value(&Coord::new(5, 1), 1);
        board.board.prefill_value(&Coord::new(0, 2), 4);
        board.board.prefill_value(&Coord::new(1, 2), 5);
        board.board.prefill_value(&Coord::new(2, 2), 0);
        board.board.prefill_value(&Coord::new(3, 2), 1);
        board.board.prefill_value(&Coord::new(4, 2), 2);
        board.board.prefill_value(&Coord::new(5, 2), 3);
        board.board.prefill_value(&Coord::new(0, 3), 1);
        board.board.prefill_value(&Coord::new(1, 3), 0);
        board.board.prefill_value(&Coord::new(2, 3), 3);
        board.board.prefill_value(&Coord::new(3, 3), 2);
        board.board.prefill_value(&Coord::new(4, 3), 5);
        board.board.prefill_value(&Coord::new(5, 3), 4);
        board.board.prefill_value(&Coord::new(0, 4), 3);
        board.board.prefill_value(&Coord::new(1, 4), 2);
        board.board.prefill_value(&Coord::new(0, 5), 5);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }

    #[test]
    fn solve_2_by_1_puzzle() {
        let mut board = RectangularBoard::new(2, 1);
        board.board.prefill_value(&Coord::new(0, 0), 0);

        assert_eq!(board.is_solved(), false);

        let solutions = board.solve();

        assert_eq!(board.is_solved(), false);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions.get(0).map(|it| it.is_solved()), Some(true));
    }
}
