use array2d::Array2D;
use itertools::Itertools;
use crate::{DIRECTIONS, PositionI32, read_to_string};

pub fn solve() {
    let content = read_to_string("21-full.txt");
    let grid = Array2D::from_rows(content.lines().map(|line| line.chars().collect_vec()).collect_vec().as_slice()).expect("correct grid");
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    let mut current = &mut Array2D::filled_with(false, y_max, x_max);
    let mut next = &mut Array2D::filled_with(false, y_max, x_max);
    for y in 0..y_max {
        for x in 0..x_max {
            if grid[(y,x)] == 'S' {
                current[(y, x)] = true;
            }
        }
    }
    let mut total = 0;
    for step in 1..=64 {
        total = 0;
        for y in 0..y_max {
            for x in 0..x_max {
                let position = PositionI32 { x: x as i32, y: y as i32 };
                next[(y,x)] = grid[(y,x)] != '#' && DIRECTIONS.iter().any(|direction| {
                    let near = position + *direction;
                    0 <= near.x && near.x < x_max as i32 && 0 <= near.y && near.y < y_max as i32 && current[(near.y as usize, near.x as usize)]
                });
                if next[(y,x)] {
                    total += 1;
                }
            }
        }
        let temp = current;
        current = next;
        next = temp;
        println!("After step {step}: {total}")
    }
    println!("{total}");
}