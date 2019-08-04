#[allow(unused)]
#[derive(Copy, Clone, Debug)]
struct Cell {
    possible_values: [bool; 9],
}

#[allow(unused)]
impl Cell {
    fn new() -> Self {
        Cell {
            possible_values: [true; 9]
        }
    }

    fn strike_through(&mut self, index: usize) -> &mut Self {
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
        if value >= 9 {
            panic!("Illegal value {}, must be smaller than 9", value);
        }
        for i in 0..9 {
            self.possible_values[i] = i == value
        }
    }
}

#[derive(PartialEq, Eq)]
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
struct Group(Vec<Coord>);

#[allow(unused)]
impl Group {
    fn new (elements: Vec<Coord>) -> Self {
        if elements.len() != 9 {
            panic!("Wrong Group size, expected 9, but got {}", elements.len());
        }
        Group(elements)
    }

    fn contains_coord(&self, coord: &Coord) -> bool {
        self.0.contains(coord)
    }
}

#[allow(unused)]
struct Board {
    cells: [Cell; 81],
    groups: Vec<Group>
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
            cells: [Cell::new(); 81],
            groups: groups
        }
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        &self.cells[coord.y * 9 + coord.x]
    }
    
    fn get_mut_cell(&mut self, coord: &Coord) -> &mut Cell {
        &mut self.cells[coord.y * 9 + coord.x]
    }

    fn set_value(&mut self, coord: &Coord, value: usize) -> &mut Self {
        let mut cells = self.cells;
        let groups: Vec<&Group> = self.groups.iter().filter(|g| { g.contains_coord(coord) }).collect();
        for g in groups {
            for i in 0..9 {
                let cur = &g.0[1];
                let mut cell = get_mut_cell(&mut cells, &cur);
                if cur == coord {
                    cell.set_value(value);
                } else {
                    cell.strike_through(value);
                }
            }
        }
        self
    }
}

fn get_mut_cell<'a>(cells: &'a mut [Cell; 81], coord: &Coord) -> &'a mut Cell {
    &mut cells[coord.y * 9 + coord.x]
}


#[cfg(test)]
mod tests {
    use super::Cell;
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
}
