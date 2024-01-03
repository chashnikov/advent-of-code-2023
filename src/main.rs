use std::{env, fmt, fs, ops};
use std::fmt::Formatter;
use std::path::Path;
use array2d::Array2D;
use itertools::Itertools;

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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

fn main() {
  let option = env::args().nth(1);
  match option.map(|s| s.parse::<i32>().unwrap_or(0)).unwrap_or(0) {
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
    12 => day12::solve(),
    13 => day13::solve(),
    14 => day14::solve(),
    15 => day15::solve(),
    16 => day16::solve(),
    17 => day17::solve(),
    18 => day18::solve(),
    19 => day19::solve(),
    20 => day20::solve(),
    _ => day21::solve(),
  }
}

pub fn read_to_string(file_name: &str) -> String {
  fs::read_to_string(Path::new("inputs").join(file_name)).expect("input must exist")
}

pub fn string_to_grid(s: &str) -> Array2D<char> {
  Array2D::from_rows(s.lines().map(|line| line.chars().collect_vec()).collect_vec().as_slice()).expect("correct grid")
}

pub fn string_to_numbers(s: &str) -> Vec<u64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}

pub fn string_to_i64_numbers(s: &str) -> Vec<i64> {
  s.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse().unwrap()).collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Direction {
  dx: i32,
  dy: i32
}

pub const NORTH: Direction = Direction { dx: 0, dy: -1 };
pub const SOUTH: Direction = Direction { dx: 0, dy: 1 };
pub const EAST: Direction = Direction { dx: 1, dy: 0 };
pub const WEST: Direction = Direction { dx: -1, dy: 0 };

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PositionI32 {
  x: i32,
  y: i32
}

impl ops::Add<Direction> for PositionI32 {
  type Output = PositionI32;
  fn add(self, rhs: Direction) -> PositionI32 {
    PositionI32 {
      x: self.x + rhs.dx,
      y: self.y + rhs.dy
    }
  }
}

impl ops::Mul<i32> for Direction {
  type Output = Direction;

  fn mul(self, rhs: i32) -> Self::Output {
    Direction {
      dx: self.dx*rhs,
      dy: self.dy*rhs
    }
  }
}

impl Direction {
  fn opposite(self) -> Direction {
    Direction {
      dx: -self.dx,
      dy: -self.dy,
    }
  }

  #[allow(dead_code)]
  fn turn_right(self) -> Direction {
    Direction {
      dx: -self.dy,
      dy: self.dx
    }
  }
}

impl ops::Sub<PositionI32> for PositionI32 {
  type Output = Direction;
  fn sub(self, rhs: PositionI32) -> Direction {
    Direction {
      dx: self.x - rhs.x,
      dy: self.y - rhs.y
    }
  }
}

impl fmt::Display for PositionI32 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.dx, self.dy)
  }
}

pub const DIRECTIONS: [Direction; 4] = [EAST, NORTH, WEST, SOUTH];

#[allow(dead_code)]
fn print_grid(grid: &Array2D<char>) {
  grid.rows_iter().for_each(|line| {
    println!("{}", line.collect::<String>())
  });
}
