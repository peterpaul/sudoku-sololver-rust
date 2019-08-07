use std::fmt;

#[allow(unused)]
#[derive(Copy, Clone)]
struct Cell {
    possible_values: [bool; 9],
    is_set: bool,
}

impl fmt::Debug for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.get_value() {
            Some(v) => v.fmt(formatter),
            None => "-".fmt(formatter),
        }
    }
}

#[allow(unused)]
impl Cell {
    fn new() -> Self {
        Cell {
            possible_values: [true; 9],
            is_set: false,
        }
    }

    fn strike_through(&mut self, index: usize) -> &mut Self {
        if self.is_set && self.get_value() == Some(index) {
            panic!("Cannot strikethrough a set value.");
        }
        self.possible_values[index] = false;
        self
    }

    fn get_value(&self) -> Option<usize> {
        let mut values = 0;
        let mut last_value = 0;
        for i in 0..9 {
            if self.possible_values[i] {
                values += 1;
                last_value = i;
            }
        }
        if values == 1 {
            Some(last_value)
        } else {
            None
        }
    }

    fn set_value(&mut self, value: usize) {
        self.prefill_value(value);
        self.is_set = true
    }

    fn prefill_value(&mut self, value: usize) {
        if value >= 9 {
            panic!("Illegal value {}, must be smaller than 9", value);
        }
        for i in 0..9 {
            self.possible_values[i] = i == value
        }
    }

    fn possibilities(&self) -> usize {
        let mut p = 0;
        for i in 0..9 {
            if self.possible_values[i] {
                p += 1;
            }
        }
        p
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {
            x: x,
            y: y,
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct Group {
    coordinates: Vec<Coord>
}

#[allow(unused)]
impl Group {
    fn new (elements: Vec<Coord>) -> Self {
        if elements.len() != 9 {
            panic!("Wrong Group size, expected 9, but got {}", elements.len());
        }
        Group {
            coordinates: elements,
        }
    }

    fn contains_coord(&self, coord: &Coord) -> bool {
        self.coordinates.contains(coord)
    }
}

#[derive(Clone)]
struct CellContainer {
    cells: [Cell; 81],
}

#[allow(unused)]
impl CellContainer {
    fn new(cells: [Cell; 81]) -> Self {
        CellContainer {
            cells: cells,
        }
    }

    fn print(&self) {
        for y in 0..9 {
            let row: Vec<String> = self.cells[(y*9)..((y+1)*9)]
                .iter()
                .map(|c| {
                    match c.get_value() {
                        Some(v) => (v + 1).to_string(),
                        None => String::from(" "),
                    }
                })
                .collect();
            println!("{:?}", row);
        }
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        &self.cells[coord.y * 9 + coord.x]
    }

    fn get_mut_cell(&mut self, coord: &Coord) -> &mut Cell {
        &mut self.cells[coord.y * 9 + coord.x]
    }

    fn get_cell_coords_to_update(&self) -> Vec<Coord> {
        let mut cell_coords_to_update = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
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

impl fmt::Debug for CellContainer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.cells[..].fmt(formatter)
    }
}

#[allow(unused)]
#[derive(Clone)]
struct Board {
    cells: CellContainer,
    groups: Vec<Group>
}

impl fmt::Debug for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.cells.fmt(formatter)
    }
}

#[allow(unused)]
impl Board {
    fn new() -> Self {
        let mut groups = Vec::new();
        for x in 0..9 {
            let mut coords = Vec::new();
            for y in 0..9 {
                coords.push(Coord::new(x, y));
            }
            groups.push(Group::new(coords));
        }
        for y in 0..9 {
            let mut coords = Vec::new();
            for x in 0..9 {
                coords.push(Coord::new(x, y));
            }
            groups.push(Group::new(coords));
        }
        for xx in 0..3 {
            for yy in 0..3 {
                let mut coords = Vec::new();
                for x in 0..3 {
                    for y in 0..3 {
                        coords.push(Coord::new(xx * 3 + x, yy * 3 + y));
                    }
                }
                groups.push(Group::new(coords));
            }
        }
        Board {
            cells: CellContainer::new([Cell::new(); 81]),
            groups: groups
        }
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        self.cells.get_cell(coord)
    }

    fn set_value(&mut self, coord: &Coord, value: usize) -> &mut Self {
        self.set_value_by(coord, value, |cell, value| { cell.set_value(value) })
    }

    fn prefill_value(&mut self, coord: &Coord, value: usize) -> &mut Self {
        self.set_value_by(coord, value, |cell, value| { cell.prefill_value(value) })
    }

    fn set_value_by(&mut self, coord: &Coord, value: usize, setter: fn(&mut Cell, usize)) -> &mut Self {
        let cells = &mut self.cells;
        let groups: Vec<&Group> = self.groups
            .iter()
            .filter(|g| { g.contains_coord(coord) })
            .collect();
        for g in groups {
            for i in 0..9 {
                let cur = &g.coordinates[i];

                let cell = cells.get_mut_cell(&cur);
                if cur == coord {
                    setter(cell, value);
                } else {
                    cell.strike_through(value);
                }
            }
        }
        self
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
                cell = self.cells.get_cell(&coord);
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

    fn is_solved(&self) -> bool {
        let mut is_solved = true;
        for y in 0..9 {
            for x in 0..9 {
                is_solved &= self.get_cell(&Coord::new(x, y)).is_set
            }
        }
        is_solved
    }

    fn solve(&self) -> Vec<Self> {
        let mut puzzle = self.clone();
        puzzle.discover_new_values();
        let pivot = puzzle.find_pivot_coord();
        let mut result = Vec::new();
        if puzzle.is_solved() {
            result.push(puzzle);
        } else {
            if let Some(p) = pivot {
                let pivot_cell = puzzle.get_cell(&p);
                for i in 0..9 {
                    if pivot_cell.possible_values[i] {
                        let mut subpuzzle = puzzle.clone();
                        subpuzzle.cells.get_mut_cell(&p).set_value(i);
                        for p in subpuzzle.solve() {
                            result.push(p);
                        }
                    }
                }
            }
        }
        result
    }

    fn find_pivot_coord(&self) -> Option<Coord> {
        let mut open_cells: Vec<(Coord, usize)> = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
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
            Some(v) => {
                Some(v.0)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use super::Board;
    use super::Coord;

    #[test]
    fn strike_through_forward_works() {
        let mut cell = Cell::new();
        for i in 0..8 {
            assert_eq!(cell.get_value(), None);
            cell.strike_through(i);
        }
        assert_eq!(cell.get_value(), Some(8));
    }

    #[test]
    fn strike_through_backward_works() {
        let mut cell = Cell::new();
        for i in (1..9).rev() {
            assert_eq!(cell.get_value(), None);
            cell.strike_through(i);
        }
        assert_eq!(cell.get_value(), Some(0));
    }

    #[test]
    #[should_panic]
    fn strike_through_set_value() {
        let mut cell = Cell::new();
        cell.set_value(4);
        cell.strike_through(4);
    }

    #[test]
    fn cell_set_value_works() {
        let mut cell = Cell::new();
        cell.set_value(4);
        assert_eq!(cell.get_value(), Some(4));
    }

    #[test]
    fn test_board() {
        let mut board = Board::new();
        board.set_value(&Coord::new(3, 3), 3);
        assert_eq!(board.get_cell(&Coord::new(3, 3)).get_value(), Some(3));
    }

    #[test]
    fn solve_puzzle() {
        let mut board = Board::new();
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
        assert_eq!(solutions.get(0).and_then(|it| Some(it.is_solved())), Some(true));
    }
}
