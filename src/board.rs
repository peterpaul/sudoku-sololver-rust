use rayon::prelude::*;

use super::cell_container::CellContainer;
use super::coord::Coord;
use super::cell::Cell;
use super::group::Group;

#[derive(Clone)]
pub struct Board {
    pub cells: CellContainer,
    pub groups: Vec<Group>
}

impl Board {
    pub fn new(cells: CellContainer, groups: Vec<Group>) -> Self {
        Board {
            cells,
            groups,
        }
    }

    pub fn get_cell(&self, coord: &Coord) -> &Cell {
        self.cells.get_cell(coord)
    }

    fn group_size(&self) -> usize {
        self.cells.group_size()
    }

    pub fn set_value(&mut self, coord: &Coord, value: usize) {
        self.set_value_by(coord, value, |cell, value, set_value| {
            if set_value {
                cell.set_value(value);
            } else {
                cell.strike_through(value);
            }
        });
    }

    pub fn prefill_value(&mut self, coord: &Coord, value: usize) {
        self.set_value_by(coord, value, |cell, value, set_value| {
            if set_value {
                cell.prefill_value(value);
            }
        });
    }

    fn set_value_by(&mut self, coord: &Coord, value: usize, setter: fn(&mut Cell, usize, bool)) {
        let cells = &mut self.cells;
        let groups: Vec<&Group> = self.groups
            .iter()
            .filter(|g| { g.contains_coord(coord) })
            .collect();
        for g in groups {
            for cur in &g.coordinates {
                let cell = cells.get_mut_cell(&cur);
                setter(cell, value, cur == coord);
            }
        }
    }

    fn discover_new_values(&mut self) -> Result<(), String> {
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
                None => return Err(String::from("Value conflict detected, no solution for this puzzle")),
            };
        }
        if discovered_new_values {
            self.discover_new_values()
        } else {
            Ok(())
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

    fn is_valid_group(&self, group: &Group) -> bool {
        let mut validation_cell = Cell::new(self.group_size());
        for coord in &group.coordinates {
            if let Some(value) = self.get_cell(&coord).get_value() {
                validation_cell.strike_through(value);
            }
        }
        validation_cell.possibilities() == 0
    }

    pub fn is_valid_solution(&self) -> bool {
        self.groups.iter()
            .all(|group| self.is_valid_group(group))
    }

    pub fn solve(&self) -> Vec<Self> {
        let mut puzzle = self.clone();
        match puzzle.discover_new_values() {
            Ok(()) => {
                if puzzle.is_solved() {
                    vec![puzzle]
                } else {
                    let pivot = puzzle.find_pivot_coord();
                    match pivot {
                        Some(p) => {
                            let pivot_cell = puzzle.get_cell(&p);
                            (0..self.group_size()).into_par_iter().flat_map(|i| {
                                let i = i as usize;
                                if pivot_cell.possible_values[i] {
                                    let mut subpuzzle = puzzle.clone();
                                    subpuzzle.set_value(&p, i);
                                    subpuzzle.solve()
                                } else {
                                    Vec::new()
                                }
                            })
                                .collect()
                        },
                        None => {
                            Vec::new()
                        }
                    }
                }
            },
            Err(_msg) => {
                // println!("{}", msg);
                Vec::new()
            }
        }
    }

    pub fn count_solutions(self) -> usize {
        let mut puzzle = self;
        match puzzle.discover_new_values() {
            Ok(()) => {
                if puzzle.is_solved() {
                    1
                } else {
                    let pivot = puzzle.find_pivot_coord();
                    match pivot {
                        Some(p) => {
                            let pivot_cell = puzzle.get_cell(&p);
                            (0..puzzle.group_size()).into_par_iter().map(|i| {
                                let i = i as usize;
                                if pivot_cell.possible_values[i] {
                                    let mut subpuzzle = puzzle.clone();
                                    subpuzzle.set_value(&p, i);
                                    subpuzzle.count_solutions()
                                } else {
                                    0
                                }
                            })
                                .sum()
                        },
                        None => {
                            0
                        }
                    }
                }
            },
            Err(_msg) => {
                // println!("{}", msg);
                0
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
