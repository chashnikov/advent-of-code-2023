use regex::Regex;
use crate::{Direction, EAST, NORTH, PositionI32, read_to_string, SOUTH, WEST};

pub fn solve() {
    let content = read_to_string("18-full.txt");
    let re = Regex::new(r"([A-Z]) +(\d+) +\(#([0-9a-f]+)\)").unwrap();
    let mut moves : Vec<(Direction, u32)> = Vec::new();
    let mut positions : Vec<PositionI32> = Vec::new();
    let mut current = PositionI32 { x:0, y:0 };
    positions.push(current.clone());
    content.lines().for_each(|line| {
        let cap = re.captures(line).unwrap();
        let coded = cap[3].chars().as_str();
        let direction = parse_direction(coded.chars().nth(5).unwrap());
        let length = u32::from_str_radix(&coded[0..5], 16).expect("must be integer");
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
    for (i, (direction, length)) in moves.iter().enumerate() {
        let delta = match *direction {
            NORTH =>  (x_max - current.x) - (current.x - x_min),
            EAST => (y_max - current.y) - (current.y - y_min),
            SOUTH => (current.x - x_min) - (x_max - current.x),
            WEST => (current.y - y_min) - (y_max - current.y),
            _ => panic!("unknown direction {}", direction)
        };
        total += delta as i64 * (*length as i64);
        current = current + *direction * *length as i32;

        let next_direction = moves[(i+1) % moves.len()].0;
        if next_direction == *direction {
            total += 2;
        }
        else if next_direction == direction.opposite() {
            panic!("turn around at move {}", i);
        }
        else if next_direction == direction.turn_right() {
            total += 3;
        }
        else {
            total += 1;
        }
        total += (*length as i64 - 1)*2;
    }
    let answer = total / 4;
    println!("{answer}");
}

fn parse_direction(c: char) -> Direction {
    match c {
        '3' => NORTH,
        '1' => SOUTH,
        '0' => EAST,
        '2' => WEST,
        _ => panic!("unknown direction {}", c)
    }
}