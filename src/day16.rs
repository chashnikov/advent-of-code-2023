use array2d::Array2D;
use crate::{Direction, EAST, NORTH, PositionI32, read_to_string, SOUTH, string_to_grid, WEST};

pub fn solve() {
    let content = read_to_string("16-full.txt");
    let grid: Array2D<char> = string_to_grid(&content);
    let mut queue : Vec<Beam> = Vec::new();
    let mut processed: Array2D<u8> = Array2D::filled_with(0, grid.num_rows(), grid.num_columns());
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    queue.push(Beam { position: PositionI32{ x: 0, y: 0}, direction: EAST});
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
    let answer : u32 = processed.rows_iter().map(|row| row.filter(|w| **w != 0).count() as u32).sum();
    println!("{answer}");
}

const DIRECTIONS: [Direction; 4] = [EAST, NORTH, WEST, SOUTH];
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
