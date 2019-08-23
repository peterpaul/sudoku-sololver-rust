use super::repeater::Repeater;

/// Captures the possible values of a single Cell.
#[derive(Clone)]
pub struct Cell {
    pub possible_values: Box<[bool]>,
    pub is_set: bool,
}

impl Cell {
    pub fn new(size: usize) -> Self {
        let possible_values: Vec<bool> = Repeater::new(Box::new(|| true))
            .take(size)
            .collect();
        Cell {
            possible_values: possible_values.into_boxed_slice(),
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
        let mut values = 0;
        let mut last_value = 0;
        for i in 0..self.len() {
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
        let mut p = 0;
        for i in 0..self.len() {
            if self.possible_values[i] {
                p += 1;
            }
        }
        p
    }
}
