use std::fs;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
  day7::solve()
}

pub fn read_to_string(file_name: &str) -> String {
  fs::read_to_string(Path::new(file_name)).expect("input must exist")
}
pub fn string_to_numbers(s: &str) -> Vec<u64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}