use std::{thread::sleep, time::Duration};

fn main() {
    let mut grid = vec![
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
        vec![true, false, true, false, true, false, true, false],
    ];

    loop {
        std::process::Command::new("clear").status().unwrap();
        show_grid(&grid);
        process_grid(&mut grid);
        sleep(Duration::from_millis(500));
    }

}

fn process_grid(grid: &mut [Vec<bool>]) {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for i in 0..rows {
        for k in 0..cols {
            grid[i][k] = !grid[i][k];
        }
    }
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
