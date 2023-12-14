use std::fmt;
use std::fmt::Formatter;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("11-full.txt");
    let mut positions: Vec<Position> = content.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)|
            Position { x: x as i64, y: y as i64 }
        )
    }).collect();

    println!("Positions: {}", positions_to_string(&positions));
    let mut dx : i64 = 0;
    let mut prev_x = -1;
    let multiplier = 1000_000;
    positions.sort_by_key(|p| p.x);
    println!("Sorted: {}", positions_to_string(&positions));
    let mut expanded_x: Vec<Position> = Vec::new();
    for p in positions {
        if p.x > prev_x {
            dx += (multiplier-1) * (p.x - prev_x - 1);
            prev_x = p.x;
        }
        expanded_x.push(Position { x: p.x + dx, y: p.y });
    }
    println!("Expanded x: {}", positions_to_string(&expanded_x));

    expanded_x.sort_by_key(|p| p.y);
    println!("Sorted: {}", positions_to_string(&expanded_x));
    let mut expanded: Vec<Position> = Vec::new();
    let mut dy : i64 = 0;
    let mut prev_y = -1;
    for p in expanded_x {
        if p.y > prev_y {
            dy += (multiplier-1) * (p.y - prev_y - 1);
            prev_y = p.y;
        }
        expanded.push(Position { x: p.x, y: p.y + dy });
    }

    println!("Expanded: {}", positions_to_string(&expanded));
    let mut answer = 0;
    for i in 0..expanded.len() {
        for j in i+1..expanded.len() {
            answer += (expanded[i].x - expanded[j].x).abs() + (expanded[i].y - expanded[j].y).abs()
        }
    }

    println!("{answer}")
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn positions_to_string(positions: &Vec<Position>) -> String {
    positions.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", ")
}