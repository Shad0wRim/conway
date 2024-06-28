use crate::cell::Cell;
use std::fmt::Display;

#[derive(Debug)]
pub struct Field<C: Cell> {
    size: (usize, usize),
    field: Vec<C>,
}

impl<C: Cell> Field<C> {
    pub fn new(size: (usize, usize)) -> Field<C> {
        Field {
            size,
            field: vec![C::default(); size.0 * size.1],
        }
    }
    pub fn next_generation(&mut self) {
        let locs_cells = self.locs().zip(self.cells());
        let mut new_states: Vec<C::State> = Vec::new();
        for (loc, cell) in locs_cells {
            new_states.push(cell.next_state(self, loc));
        }
        for (index, state) in new_states.into_iter().enumerate() {
            self.field[index].update(state);
        }
    }
    pub fn get(&self, loc: (usize, usize)) -> Option<&C> {
        let index = self.index(loc);
        self.field.get(index)
    }
    pub fn get_mut(&mut self, loc: (usize, usize)) -> Option<&mut C> {
        let index = self.index(loc);
        self.field.get_mut(index)
    }
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn locs(&self) -> impl Iterator<Item = (usize, usize)> {
        let (length, height) = self.size;
        (0..height).flat_map(move |y| (0..length).map(move |x| (x, y)))
    }
    pub fn cells(&self) -> impl Iterator<Item = &C> {
        let (length, height) = self.size;
        (0..length * height).map(|ind| &self.field[ind])
    }
    pub fn index(&self, (x, y): (usize, usize)) -> usize {
        y * self.size.0 + x
    }
}
impl<C: Cell> Default for Field<C> {
    fn default() -> Self {
        Self::new((1, 1))
    }
}
impl<C: Cell> Display for Field<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (num, cell) in self.cells().enumerate() {
            output.push_str(&cell.to_string());
            if num % self.size.0 == self.size.0 - 1 {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}
