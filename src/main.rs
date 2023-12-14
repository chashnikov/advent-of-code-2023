use std::{env, fs};
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
  match env::args().next().map(|s| s.parse::<i32>().unwrap_or(0)).unwrap_or(0) {
    1 => day1::solve(),
    2 => day2::solve(),
    3 => day3::solve(),
    4 => day4::solve(),
    5 => day5::solve(),
    6 => day6::solve(),
    7 => day7::solve(),
    8 => day8::solve(),
    9 => day9::solve(),
    10 => day10::solve(),
    11 => day11::solve(),
    _ => day12::solve()
  }
}

pub fn read_to_string(file_name: &str) -> String {
  fs::read_to_string(Path::new(file_name)).expect("input must exist")
}
pub fn string_to_numbers(s: &str) -> Vec<u64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}

pub fn string_to_i64_numbers(s: &str) -> Vec<i64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}