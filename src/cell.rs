use crate::Field;
use std::fmt::{Debug, Display};

pub trait Cell: Display + Copy + Default {
    type State: Debug;
    fn update(&mut self, state: Self::State);
    fn next_state(&self, field: &Field<Self>, loc: (usize, usize)) -> Self::State;
}
