mod tui;

use std::time::Duration;

use conway::{ConwayCell, Field};
use ratatui::{
    crossterm::event::{self, Event, MouseButton},
    prelude::*,
    widgets::{canvas::Canvas, Block},
    Frame,
};
use tui::Tui;

fn main() -> std::io::Result<()> {
    tui::panic_hook();
    let mut terminal = tui::init()?;
    let size = terminal.size()?.as_size();
    let app = App::new(((size.width - 2).into(), (size.height - 2).into()));
    let res = run_app(&mut terminal, app);
    tui::restore()?;
    if let Err(e) = res {
        println!("{e:?}");
    }
    Ok(())
}

fn run_app(terminal: &mut Tui, mut app: App) -> std::io::Result<()> {
    loop {
        app.update();
        terminal.draw(|f| ui(f, &app))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) => match app.mode {
                InputMode::Edit => match key.code {
                    event::KeyCode::Enter => app.start(),
                    event::KeyCode::Char('h') | event::KeyCode::Left => app.move_cursor_left(),
                    event::KeyCode::Char('l') | event::KeyCode::Right => app.move_cursor_right(),
                    event::KeyCode::Char('k') | event::KeyCode::Up => app.move_cursor_up(),
                    event::KeyCode::Char('j') | event::KeyCode::Down => app.move_cursor_down(),
                    event::KeyCode::Char(' ') => app.toggle(),
                    event::KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                InputMode::Run => match key.code {
                    event::KeyCode::Esc => return Ok(()),
                    event::KeyCode::Enter => app.pause(),
                    event::KeyCode::Char('h') | event::KeyCode::Left => app.move_cursor_left(),
                    event::KeyCode::Char('l') | event::KeyCode::Right => app.move_cursor_right(),
                    event::KeyCode::Char('k') | event::KeyCode::Up => app.move_cursor_up(),
                    event::KeyCode::Char('j') | event::KeyCode::Down => app.move_cursor_down(),
                    _ => {}
                },
            },
            Event::Mouse(mouse) => match app.mode {
                InputMode::Edit => match mouse.kind {
                    event::MouseEventKind::Down(button) if button == MouseButton::Left => {
                        app.set_cursor(mouse.row, mouse.column);
                        app.toggle_dragging_state();
                        app.dragging_loc = app.cursor_loc;
                        app.set_cell();
                    }
                    event::MouseEventKind::Drag(button) if button == MouseButton::Left => {
                        app.set_cursor(mouse.row, mouse.column);
                        app.set_cell();
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let canvas = Canvas::default()
        .block(Block::bordered().title("Game of Life"))
        .marker(symbols::Marker::Dot)
        .paint(|ctx| ctx.draw(&app.field));
    f.render_widget(canvas, f.size());
    f.set_cursor(
        (app.cursor_loc.0 + 1).try_into().unwrap(),
        (app.cursor_loc.1 + 1).try_into().unwrap(),
    );
}

struct App {
    field: Field<ConwayCell>,
    cursor_loc: (usize, usize),
    dragging_loc: (usize, usize),
    dragging_state: bool,
    mode: InputMode,
}

impl App {
    fn new(size: (usize, usize)) -> Self {
        Self {
            field: Field::new(size),
            cursor_loc: (0, 0),
            dragging_loc: (0, 0),
            dragging_state: true,
            mode: InputMode::Edit,
        }
    }
    fn move_cursor_left(&mut self) {
        self.cursor_loc.0 = self.cursor_loc.0.saturating_sub(1);
    }
    fn move_cursor_right(&mut self) {
        let (x_lim, _) = self.field.size();
        self.cursor_loc.0 += 1;
        self.cursor_loc.0 = self.cursor_loc.0.clamp(0, x_lim - 1);
    }
    fn move_cursor_down(&mut self) {
        let (_, y_lim) = self.field.size();
        self.cursor_loc.1 += 1;
        self.cursor_loc.1 = self.cursor_loc.1.clamp(0, y_lim - 1);
    }
    fn move_cursor_up(&mut self) {
        self.cursor_loc.1 = self.cursor_loc.1.saturating_sub(1);
    }
    fn start(&mut self) {
        self.mode = InputMode::Run;
    }
    fn pause(&mut self) {
        self.mode = InputMode::Edit;
    }
    fn toggle(&mut self) {
        let cell = self
            .field
            .get_mut(self.cursor_loc)
            .expect("Cursor is bounded");
        cell.alive = !cell.alive;
    }
    fn update(&mut self) {
        if let InputMode::Run = self.mode {
            self.field.next_generation();
        }
    }
    fn set_cursor(&mut self, row: u16, column: u16) {
        let (x_lim, y_lim) = self.field.size();
        self.cursor_loc = (
            (column as usize).saturating_sub(1).clamp(0, x_lim - 1),
            (row as usize).saturating_sub(1).clamp(0, y_lim - 1),
        );
    }
    fn toggle_dragging_state(&mut self) {
        self.dragging_state = !self
            .field
            .get(self.cursor_loc)
            .expect("Cursor is bounded")
            .alive;
    }
    fn set_cell(&mut self) {
        let cell = self
            .field
            .get_mut(self.cursor_loc)
            .expect("Cursor is bounded");
        cell.alive = self.dragging_state;
    }
}

enum InputMode {
    Edit,
    Run,
}

#[allow(dead_code)]
mod basic {
    use conway::{ConwayCell, Field};
    fn main() {
        let generations = 50;
        let mut game = Field::<ConwayCell>::new((20, 15));
        let alive_locs = [(1, 1), (1, 0), (1, 2)];

        for loc in alive_locs {
            game.get_mut(loc).unwrap().alive = true;
        }

        for _ in 0..generations {
            display(&game);
            game.next_generation();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    fn display(field: &impl std::fmt::Display) {
        const ED0: &str = "\x1b[J";
        const CUP: &str = "\x1b[H";
        print!("{CUP}");
        print!("{ED0}");
        print!("{field}");
    }
}
