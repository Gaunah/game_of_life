use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Frame, Terminal,
};

struct App {
    grid: Vec<Vec<bool>>,
}

impl App {
    fn new(size: usize) -> Self {
        Self {
            grid: vec![vec![false; size]; size],
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = Self::new(50);
        app.init_grid();

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(150);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.process_grid();
                last_tick = Instant::now();
            }
        }

        restore_termianl()
    }

    fn init_grid(&mut self) {
        // just a glider
        self.grid[2][1] = true;
        self.grid[3][2] = true;
        self.grid[4][0] = true;
        self.grid[4][2] = true;
        self.grid[4][1] = true;
    }

    fn ui(&self, frame: &mut Frame) {
        let area = frame.size();

        // WIP - don't judge me
        let rows = self.grid.len();
        let cols = if rows > 0 { self.grid[0].len() } else { 0 };

        let mut grid_str = String::new();
        for i in 0..rows {
            for k in 0..cols {
                if self.grid[i][k] {
                    grid_str += "O";
                } else {
                    grid_str += " ";
                }
            }
            grid_str += "\n";
        }

        frame.render_widget(Paragraph::new(grid_str).green(), area);
    }

    fn process_grid(&mut self) {
        let rows = self.grid.len();
        let cols = if rows > 0 { self.grid[0].len() } else { 0 };

        let mut next_gen = vec![vec![false; cols]; rows];
        for i in 0..rows {
            for k in 0..cols {
                match Self::number_of_neighbors(&self.grid, (i, k)) {
                    2 => next_gen[i][k] = self.grid[i][k],
                    3 => next_gen[i][k] = true,
                    _ => next_gen[i][k] = false,
                }
            }
        }

        self.grid = next_gen;
    }

    fn number_of_neighbors(grid: &[Vec<bool>], coord: (usize, usize)) -> u8 {
        // short cut at >3, because it doesn't matter
        let rows = grid.len() as isize;
        let cols = if rows > 0 { grid[0].len() } else { 0 } as isize;

        let mut sum = 0;
        for &(di, dk) in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let ni = coord.0 as isize + di;
            let nk = coord.1 as isize + dk;

            if ni >= 0 && ni < rows && nk >= 0 && nk < cols && grid[ni as usize][nk as usize] {
                sum += 1;
            }
            if sum > 3 {
                break;
            }
        }

        sum
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_termianl() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() {
    App::run().unwrap();
}
