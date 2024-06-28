pub mod cell;
pub mod field;
pub use field::Field;

use cell::Cell;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub struct ConwayCell {
    pub alive: bool,
}

impl Cell for ConwayCell {
    type State = bool;
    fn update(&mut self, state: Self::State) {
        self.alive = state;
    }
    fn next_state(&self, field: &Field<Self>, loc: (usize, usize)) -> Self::State {
        let alive_count = (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .filter(|&loc| loc != (0, 0))
            .map(|(x, y)| (x + loc.0 as i32, y + loc.1 as i32))
            .filter_map(|(x, y)| Some((x.try_into().ok()?, y.try_into().ok()?)))
            .filter(|(x, y)| *x < field.size().0 && *y < field.size().1)
            .filter_map(|loc| field.get(loc))
            .fold(0, |count, cell| count + cell.alive as u8);

        if self.alive {
            if alive_count < 2 {
                false
            } else {
                alive_count <= 3
            }
        } else if alive_count == 3 {
            true
        } else {
            self.alive
        }
    }
}

impl Display for ConwayCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.alive {
                true => '\u{2588}',
                false => ' ', //'\u{2591}',
            }
        )
    }
}

impl ratatui::widgets::canvas::Shape for Field<ConwayCell> {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        for (num, cell) in self.cells().enumerate() {
            let location = (num % self.size().0, num / self.size().0);
            match cell.alive {
                true => painter.paint(location.0, location.1, ratatui::style::Color::White),
                false => continue,
            }
        }
    }
}
