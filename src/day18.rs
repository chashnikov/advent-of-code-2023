use array2d::Array2D;
use regex::Regex;
use crate::{Direction, EAST, NORTH, PositionI32, print_grid, read_to_string, SOUTH, WEST};

pub fn solve() {
    let content = read_to_string("18-full.txt");
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
    let mut grid: Array2D<char> = Array2D::filled_with('.', (y_max -y_min + 1) as usize, (x_max - x_min + 1) as usize);
    let mut grid_inside: Array2D<char> = Array2D::filled_with('.', (y_max -y_min + 1) as usize, (x_max - x_min + 1) as usize);
    let mut inside: Array2D<bool> = Array2D::filled_with(false, (y_max -y_min + 1) as usize, (x_max - x_min + 1) as usize);
    current = PositionI32 { x: -x_min, y: -y_min };
    grid[(current.y as usize, current.x as usize)] = '#';
    for (direction, length) in moves {
        for i in 0..=length {
            let inside_position = current + direction.turn_right();
            if inside_position.x >= 0 && inside_position.x < inside.num_columns() as i32
                && inside_position.y >= 0 && inside_position.y < inside.num_rows() as i32 {
                inside[(inside_position.y as usize, inside_position.x as usize)] = true;
                grid_inside[(inside_position.y as usize, inside_position.x as usize)] = 'i';
            }
            if i < length {
                current = current + direction;
                grid[(current.y as usize, current.x as usize)] = '#';
            }
        }
    }
    print_grid(&grid);
    println!("Inside...");
    print_grid(&grid_inside);
    println!("Filling...");
    fill_grid(&mut grid, &inside);
    print_grid(&grid);
    println!();
    let answer : usize = grid.rows_iter().map(|row| {
        row.filter(|c| **c == '#').count()
    }).sum();
    println!("{answer}");
}

fn fill_grid(grid: &mut Array2D<char>, inside: &Array2D<bool>) {
    for y in 0..grid.num_rows() {
        let mut was_inside = false;
        for x in 0..grid.num_columns() {
            match grid[(y,x)] {
                '#' => was_inside = false,
                _ => {
                    was_inside = was_inside || inside[(y,x)];
                    if was_inside {
                        grid[(y,x)] = '#';
                    }
                }
            }
        }
    }
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