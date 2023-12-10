use std::{fmt, ops};
use std::fmt::Formatter;
use itertools::Itertools;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("10-full.txt");
    let grid : Vec<Vec<(Direction, Direction)>>= content.lines().map(|line| {
        line.chars().map(|c| char_to_tile(c)).collect()
    }).collect();
    let start = Position {
        x: grid.iter().map(|line| {
            line.iter().find_position(|t| **t == S_TILE).map(|(i, _)| i).unwrap_or(0)
        }).max().unwrap() as i32,
        y: grid.iter().find_position(|line| line.iter().any(|t| *t == S_TILE)).unwrap().0 as i32
    };
    let next = [NORTH, SOUTH, WEST, EAST].iter().map(|d| start + d.clone()).find(|next| {
        if let Some((d1, d2)) = index(&grid, &next) {
            *next + d1 == start || *next + d2 == start
        }
        else {
            false
        }
    }).unwrap();
    let mut prev = start;
    let mut current = next;
    let mut length = 0;
    while current != start {
        let (d1, d2) = index(&grid, &current).unwrap();
        let next = current + supplemental(&(prev - current), &d1, &d2);
        prev = current;
        current = next;
        length += 1;
    }
    let answer = (length+1)/2;
    println!("{answer}");
}

fn char_to_tile(c: char) -> (Direction, Direction) {
    match c {
        '|' => VER_TILE,
        '-' => HOR_TILE,
        'L' => L_TILE,
        'J' => J_TILE,
        '7' => SEVEN_TILE,
        'F' => F_TILE,
        'S' => S_TILE,
        '.' => GROUND_TILE,
        _ => panic!("unexpected tile char")
    }
}

fn index(grid: &Vec<Vec<(Direction, Direction)>>, position: &Position) -> Option<(Direction, Direction)> {
    if position.y < 0 || position.y >= grid.len() as i32 {
        return None;
    }
    let row = &grid[position.y as usize];
    if position.x < 0 || position.x >= row.len() as i32 {
        return None;
    }
    let tile = row[position.x as usize];
    return if tile == GROUND_TILE { None } else { Some(tile) };
}

const NORTH: Direction = Direction { dx: 0, dy: -1 };
const SOUTH: Direction = Direction { dx: 0, dy: 1 };
const EAST: Direction = Direction { dx: 1, dy: 0 };
const WEST: Direction = Direction { dx: -1, dy: 0 };
const VER_TILE: (Direction, Direction) = (NORTH, SOUTH);
const HOR_TILE: (Direction, Direction) = (EAST, WEST);
const L_TILE: (Direction, Direction) = (NORTH, EAST);
const J_TILE: (Direction, Direction) = (NORTH, WEST);
const SEVEN_TILE: (Direction, Direction) = (SOUTH, WEST);
const F_TILE: (Direction, Direction) = (SOUTH, EAST);

const S_TILE: (Direction, Direction) = (NORTH, NORTH);
const GROUND_TILE: (Direction, Direction) = (SOUTH, SOUTH);

fn supplemental(d: &Direction, d1: &Direction, d2: &Direction) -> Direction {
    Direction {
        dx: d1.dx + d2.dx - d.dx,
        dy: d1.dy + d2.dy - d.dy,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Direction {
    dx: i32,
    dy: i32
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

impl ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Position {
        Position {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Direction;
    fn sub(self, rhs: Position) -> Direction {
        Direction {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
