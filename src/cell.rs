/// Captures the possible values of a single Cell.
#[derive(Clone, Debug)]
pub struct Cell {
    pub possible_values: Box<[bool]>,
    pub is_set: bool,
}

impl Cell {
    pub fn new(size: usize) -> Self {
        Cell {
            possible_values: vec![true; size].into_boxed_slice(),
            is_set: false,
        }
    }

    pub fn strike_through(&mut self, index: usize) {
        if self.is_set && self.get_value() == Some(index) {
            panic!("Cannot strikethrough a set value.");
        }
        self.possible_values[index] = false;
    }

    fn len(&self) -> usize {
        self.possible_values.len()
    }

    pub fn get_value(&self) -> Option<usize> {
        let possible_values: Vec<usize> = self.possible_values.iter().enumerate()
            .filter_map(|(index, is_possible)| if *is_possible {
                Some(index)
            } else {
                None
            })
            .collect();
        if possible_values.len() == 1 {
            Some(possible_values[0])
        } else {
            None
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.prefill_value(value);
        self.is_set = true
    }

    pub fn prefill_value(&mut self, value: usize) {
        if value >= self.len() {
            panic!("Illegal value {}, must be smaller than {}", value, self.len());
        }
        for i in 0..self.len() {
            self.possible_values[i] = i == value
        }
    }

    pub fn possibilities(&self) -> usize {
        self.possible_values.iter()
            .filter(|is_possible| **is_possible)
            .count()
    }
}
