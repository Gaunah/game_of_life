use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let size = 30;
    let mut grid = vec![vec![false; size]; size];

    grid[2][1] = true;
    grid[3][2] = true;
    grid[4][0] = true;
    grid[4][1] = true;
    grid[4][2] = true;

    loop {
        std::process::Command::new("clear").status().unwrap();
        show_grid(&grid);
        let timer = Instant::now();
        grid = process_grid(&grid);
        let duration = timer.elapsed();
        dbg!(duration);
        sleep(Duration::from_millis(
            500_u64.saturating_sub(duration.as_millis() as u64),
        ));
    }
}

fn process_grid(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut next_gen = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for k in 0..cols {
            match number_of_neighbors(grid, (i, k)) {
                2 => next_gen[i][k] = grid[i][k],
                3 => next_gen[i][k] = true,
                _ => next_gen[i][k] = false,
            }
        }
    }

    next_gen
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

fn show_grid(grid: &[Vec<bool>]) {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for i in 0..rows {
        for k in 0..cols {
            if grid[i][k] {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
