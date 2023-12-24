use std::cmp::min;
use array2d::Array2D;
use itertools::Itertools;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("13-full.txt");
    let mut answer = 0;
    let mut grid : Vec<Vec<char>> = Vec::new();
    content.lines().for_each(|line| {
        if line.is_empty() {
            answer += total_reflections(to_array(&grid));
            grid.clear();
        }
        else {
            grid.push(line.trim().chars().collect_vec());
        }
    });
    answer += total_reflections(to_array(&grid));
    println!("{answer}");
}

fn to_array(grid: &Vec<Vec<char>>) -> Array2D<char> {
    Array2D::from_rows(grid.as_slice()).expect("correct grid")
}

fn total_reflections(grid: Array2D<char>) -> u64 {
    let answer = reflections(grid.as_columns()) + 100 * reflections(grid.as_rows());
    println!("{answer}");
    answer
}

fn reflections(lines: Vec<Vec<char>>) -> u64 {
    let mut result = 0_u64;
    for i in 1..lines.len() {
        // println!("Checking {i}:");
        if (1..=min(i, lines.len() - i)).all(|j| {
            let res = lines[i - j] == lines[i + j - 1];
            // println!(" [{}] {} [{}]", i-j, if res { " == " } else { "!=" }, i+j-1);
            res
        }) {
            result += i as u64;
        }
    }
    return result;
}