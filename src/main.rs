mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
  day6::day6()
}

pub fn string_to_numbers(s: &str) -> Vec<u64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}