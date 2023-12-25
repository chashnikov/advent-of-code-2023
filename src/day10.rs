use array2d::Array2D;
use itertools::Itertools;
use crate::{Direction, EAST, NORTH, PositionI32, read_to_string, SOUTH, WEST};

pub fn solve() {
    let content = read_to_string("10-full.txt");
    let grid : Vec<Vec<(Direction, Direction)>>= content.lines().map(|line| {
        line.chars().map(|c| char_to_tile(c)).collect()
    }).collect();
    let start = PositionI32 {
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
    let height = grid.len();
    let width = grid.first().unwrap().len();
    let mut markers : Array2D<u32> = Array2D::filled_with(0, height, width);
    while current != start {
        let (d1, d2) = index(&grid, &current).unwrap();
        let marker = if d1.dy <= 0 && d2.dy <= 0 {
            2
        } else {
            1
        };
        markers[(current.y as usize, current.x as usize)] = marker;
        let next = current + supplemental(&(prev - current), &d1, &d2);
        prev = current;
        current = next;
    }
    let start_d1 = next - start;
    let start_d2 = prev - start;
    let start_marker = if start_d1.dy <= 0 && start_d2.dy <= 0 { 2 } else { 1 };
    markers[(start.y as usize, start.x as usize)] = start_marker;
    let mut inside = 0;
    for y in 0..height {
        let mut borders = 0;
        for x in 0..width {
            let marker = markers[(y, x)];
            if marker == 0 && borders % 2 == 1 {
                inside += 1;
            }
            borders += marker;
        }
    }
    println!("{inside}");
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

fn index(grid: &Vec<Vec<(Direction, Direction)>>, position: &PositionI32) -> Option<(Direction, Direction)> {
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
