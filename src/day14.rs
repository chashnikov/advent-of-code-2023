use array2d::Array2D;
use itertools::Itertools;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("14-full.txt");
    let grid : Array2D<char> = Array2D::from_rows(content.lines().map(|line| line.chars().collect_vec()).collect_vec().as_slice()).expect("correct grid");
    let answer: u64 = grid.columns_iter().map(|column| calculate_load(&column.collect_vec())).sum();
    println!("{answer}");
}

fn calculate_load(column: &Vec<&char>) -> u64 {
    let mut load = 0_u64;
    let mut row_multiplier = column.len() as u64;
    let mut actual_multiplier = row_multiplier;
    column.iter().for_each(|c| {
        match *c {
            'O' => {
                load += actual_multiplier;
                actual_multiplier -= 1;
            }
            '#' => {
                actual_multiplier = row_multiplier - 1;
            }
            _ => {
            }
        }
        row_multiplier -= 1;
    });
    return load;
}