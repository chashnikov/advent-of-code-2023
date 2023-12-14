use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("11-full.txt");
    let merged = content.lines().map(String::from).reduce(|acc, line| {
        acc.chars().zip(line.chars()).map(|(c1, c2)| {
            if c1 == '.' && c2 == '.' { '.' } else { '#' }
        }).collect::<String>()
    }).unwrap();
    let expanded : Vec<String> = content.lines().flat_map(|line| {
        let expanded_line = line.chars().zip(merged.chars()).flat_map(|(c, m)| {
            if m == '.' { ['.', '.'].to_vec() } else { [c].to_vec() }
        }).collect::<String>();
        if expanded_line.chars().all(|c| c == '.') {
            [expanded_line.clone(), expanded_line.clone()].to_vec()
        }
        else {
            [expanded_line].to_vec()
        }
    }).map(|line| line.clone()).collect();
    for l in &expanded {
        println!("{l}")
    }
    let positions : Vec<Position> = expanded.iter().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)|
            Position { x: x as i32, y: y as i32 }
        )
    }).collect();

    let mut answer = 0;
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            answer += (positions[i].x - positions[j].x).abs() + (positions[i].y - positions[j].y).abs()
        }
    }

    println!("{answer}")
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32
}