use array2d::Array2D;
use itertools::Itertools;
use crate::{read_to_string, string_to_grid};

pub fn solve() {
    let content = read_to_string("14-full.txt");
    let initial_grid: Array2D<char> = string_to_grid(&content);
    let mut grid = initial_grid.clone();
    let mut grid_x2 = grid.clone();
    let mut cycles = 0_u64;
    loop {
        cycle(&mut grid);
        cycle(&mut grid_x2);
        cycle(&mut grid_x2);
        cycles += 1;
        if cycles % 1000 == 0 {
            print!(".");
            if cycles % 100_000 == 0 {
                println!();
            }
        }
        if grid == grid_x2 {
            println!();
            println!("Found cycle: {}", cycles);
            break;
        }
    }

    let total = 1_000_000_000;
    let remaining = total - cycles;
    for _ in 1..=(remaining % cycles) {
        cycle(&mut grid);
    }
    let answer : u64 = grid_load(&grid);
    println!("{answer}");
}

fn grid_load(grid: &Array2D<char>) -> u64 {
    grid.columns_iter().map(|column| column_load(&column.collect_vec())).sum()
}

fn column_load(column: &Vec<&char>) -> u64 {
    return column.iter().enumerate().filter(|(_, c)| ***c == 'O').map(|(i, _)| (column.len() - i) as u64).sum();
}

#[allow(dead_code)]
fn print_grid(grid: &Array2D<char>) {
    grid.rows_iter().for_each(|line| {
        println!("{}", line.collect::<String>())
    });
    println!("Load: {}", grid_load(grid))
}

fn cycle(grid: &mut Array2D<char>) {
    tilt(grid, 0, -1);
    // print_grid(&grid);
    tilt(grid, -1, 0);
    // print_grid(&grid);
    tilt(grid, 0, 1);
    // print_grid(&grid);
    tilt(grid, 1, 0);
}

fn tilt(grid: &mut Array2D<char>, dx: i32, dy: i32) {
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    let max = x_max * y_max;
    let mut x_prev = 0_i32;
    let mut y_prev = 0_i32;
    let mut rounded = 0;
    let mut total = 0;
    let mut last = true;
    for xy in 0..max {
        let x = match dx {
            -1 => { x_max - 1 - xy % x_max },
            1 => { xy % x_max },
            _ => { xy / y_max }
        };
        let y = match dy {
            -1 => { y_max - 1 - xy % y_max },
            1 => { xy % y_max },
            _ => { xy / x_max }
        };
        if last {
            x_prev = x as i32;
            y_prev = y as i32;
        }
        let cell = grid[(y, x)];
        if cell == 'O' {
            rounded += 1;
        }
        if cell != '#' {
            total += 1;
        }
        last = cell == '#' || dx == 0 && (xy + 1) % y_max == 0 || dy == 0 && (xy + 1) % x_max == 0;
        if last {
            for i in 0..total {
                let y_fill = y_prev + (total - 1 - i) * dy;
                let x_fill = x_prev + (total - 1 - i) * dx;
                grid[(y_fill as usize, x_fill as usize)] = if rounded > 0 { 'O' } else { '.' };
                rounded -= 1;
            }
            rounded = 0;
            total = 0;
        }
    }
}

