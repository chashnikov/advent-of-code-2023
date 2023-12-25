use array2d::Array2D;
use itertools::Itertools;
use crate::{Direction, DIRECTIONS, EAST, NORTH, PositionI32, read_to_string, SOUTH, string_to_grid, WEST};

pub fn solve() {
    let content = read_to_string("16-full.txt");
    let grid: Array2D<char> = string_to_grid(&content);
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    let initial : Vec<Beam> = DIRECTIONS.iter().flat_map(|direction| {
        match *direction {
            EAST => (0..y_max).map(|y| Beam { position: PositionI32 { x: 0, y: y as i32 }, direction: *direction}).collect_vec(),
            WEST => (0..y_max).map(|y| Beam { position: PositionI32 { x: (x_max - 1) as i32, y: y as i32 }, direction: *direction}).collect_vec(),
            NORTH => (0..x_max).map(|x| Beam { position: PositionI32 { x: x as i32, y: (y_max - 1) as i32 }, direction: *direction}).collect_vec(),
            SOUTH => (0..x_max).map(|x| Beam { position: PositionI32 { x: x as i32, y: 0 }, direction: *direction}).collect_vec(),
            _ => panic!("unexpected direction {}", direction)
        }
    }).collect();
    let answer = initial.iter().map(|beam| count_energized(&grid, beam)).max().unwrap();
    println!("{answer}");
}

fn count_energized(grid: &Array2D<char>, initial: &Beam) -> u32 {
    let mut queue: Vec<Beam> = Vec::new();
    let mut processed: Array2D<u8> = Array2D::filled_with(0, grid.num_rows(), grid.num_columns());
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    queue.push((*initial).clone());
    while !queue.is_empty() {
        let beam = queue.pop().unwrap();
        if beam.position.x < 0 || beam.position.x >= x_max as i32 || beam.position.y < 0 || beam.position.y >= y_max as i32 {
            continue;
        }
        let direction_mask = 1 << DIRECTIONS.iter().position(|p| *p == beam.direction).unwrap() as u8;
        let index = (beam.position.y as usize, beam.position.x as usize);
        if processed[index] & direction_mask != 0 {
            continue;
        }
        processed[index] |= direction_mask;
        let char = grid[index];
        match char {
            '.' => { queue.push(beam.position.advance(beam.direction)); }
            '/' => { queue.push(beam.position.advance(reflect_by_slash(beam.direction))); }
            '\\' => { queue.push(beam.position.advance(reflect_by_back_slash(beam.direction))); }
            '-' => {
                let (main, additional) = split_by_horizontal(beam.direction);
                queue.push(beam.position.advance(main));
                if let Some(additional) = additional {
                    queue.push(beam.position.advance(additional));
                }
            }
            '|' => {
                let (main, additional) = split_by_vertical(beam.direction);
                queue.push(beam.position.advance(main));
                if let Some(additional) = additional {
                    queue.push(beam.position.advance(additional));
                }
            }
            _ => panic!("unexpected char {}", char)
        }
    }
    print_grid(&processed);
    let answer: u32 = processed.rows_iter().map(|row| row.filter(|w| **w != 0).count() as u32).sum();
    answer
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Beam {
    position: PositionI32,
    direction: Direction,
}

fn reflect_by_slash(direction: Direction) -> Direction {
    match direction {
        EAST => NORTH,
        NORTH => EAST,
        WEST => SOUTH,
        SOUTH => WEST,
        _ => panic!("unexpected direction {}", direction)
    }
}

fn reflect_by_back_slash(direction: Direction) -> Direction {
    match direction {
        EAST => SOUTH,
        NORTH => WEST,
        WEST => NORTH,
        SOUTH => EAST,
        _ => panic!("unexpected direction {}", direction)
    }
}

fn split_by_horizontal(direction: Direction) -> (Direction, Option<Direction>) {
    match direction {
        EAST => (EAST, None),
        WEST => (WEST, None),
        NORTH => (EAST, Some(WEST)),
        SOUTH => (EAST, Some(WEST)),
        _ => panic!("unexpected direction {}", direction)
    }
}

fn split_by_vertical(direction: Direction) -> (Direction, Option<Direction>) {
    match direction {
        NORTH => (NORTH, None),
        SOUTH => (SOUTH, None),
        EAST => (NORTH, Some(SOUTH)),
        WEST => (NORTH, Some(SOUTH)),
        _ => panic!("unexpected direction {}", direction)
    }
}

impl PositionI32 {
    fn advance(self, direction: Direction) -> Beam {
        Beam {
            position: self + direction,
            direction
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Array2D<u8>) {
    grid.rows_iter().for_each(|line| {
        println!("{}", line.map(|w| format!("{:x}", w)).collect::<String>())
    });
}
