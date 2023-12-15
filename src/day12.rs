use array2d::Array2D;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("12.txt");
    let answer : u64 = content.lines().map(|line| number_of_arrangements(line)).sum();
    println!("{answer}")
}

fn number_of_arrangements(line: &str) -> u64 {
    let (pattern, numbers_string) = line.split_once(' ').unwrap();
    let numbers : Vec<u64> = numbers_string.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
    let result: Array2D<u64> = Array2D::filled_with(0, pattern.len() + 1, numbers.len() + 1);
    result[(0,0)] = 1;
    for p in 1..=pattern.len() {
        for n in 1..=numbers.len() {

        }
    }
    return 0
}