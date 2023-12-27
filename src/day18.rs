use regex::Regex;
use crate::{Direction, EAST, NORTH, PositionI32, read_to_string, SOUTH, WEST};

pub fn solve() {
    let content = read_to_string("18-min.txt");
    let re = Regex::new(r"([A-Z]) +(\d+) +\(#[0-9a-f]+\)").unwrap();
    let mut moves : Vec<(Direction, u32)> = Vec::new();
    let mut positions : Vec<PositionI32> = Vec::new();
    let mut current = PositionI32 { x:0, y:0 };
    positions.push(current.clone());
    content.lines().for_each(|line| {
        let cap = re.captures(line).unwrap();
        let direction = parse_direction(cap[1].chars().next().unwrap());
        let length = cap[2].parse::<u32>().unwrap();
        moves.push((direction, length));
        current = current + direction * length as i32;
        positions.push(current);
    });
    let x_min = positions.iter().map(|p| p.x).min().unwrap();
    let x_max = positions.iter().map(|p| p.x).max().unwrap();
    let y_min = positions.iter().map(|p| p.y).min().unwrap();
    let y_max = positions.iter().map(|p| p.y).max().unwrap();
    println!("({}, {}) x ({}, {})", x_min, x_max, y_min, y_max);
    let mut total = 0_i64;
    current = PositionI32 { x:0, y:0 };
    for (direction, length) in moves {
        let delta = match direction {
            NORTH =>  (x_max + 1 - current.x) - (current.x - x_min),
            EAST => (y_max + 1 - current.y) - (current.y - y_min),
            SOUTH => (current.x + 1 - x_min) - (x_max - current.x),
            WEST => (current.y + 1 - y_min) - (y_max - current.y),
            _ => panic!("unknown direction {}", direction)
        };
        total += delta as i64 * (length as i64 + 1);
        current = current + direction * length as i32;
    }
    let answer = total / 4;
    println!("{answer}");
}

fn parse_direction(c: char) -> Direction {
    match c {
        'U' => NORTH,
        'D' => SOUTH,
        'R' => EAST,
        'L' => WEST,
        _ => panic!("unknown direction {}", c)
    }
}