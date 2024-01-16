use std::cmp::min;
use array2d::Array2D;
use itertools::Itertools;
use crate::{DIRECTIONS, PositionI32, read_to_string};

pub fn solve() {
    let content = read_to_string("21-full.txt");
    let steps = 73;
    // let steps = 26501365;

    let grid = Array2D::from_rows(content.lines().map(|line| line.chars().collect_vec()).collect_vec().as_slice()).expect("correct grid");
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();

    let mut x_s = 0;
    let mut y_s = 0;
    'outer: for y in 0..y_max {
        for x in 0..x_max {
            if grid[(y,x)] == 'S' {
                x_s = x;
                y_s = y;
                break 'outer;
            }
        }
    }
    let mut total = 0_u64;
    assert_eq!(x_max % 2, 1);
    assert_eq!(steps % 2, 1);
    assert_eq!(x_max, y_max);
    assert_eq!(2*x_s+1, x_max);
    let mut totals : [u64; 4] = [0, 0, 0, 0];
    let mut corner_index = 0;
    for dx in [-1_i32, 1] {
        for dy in [-1_i32, 1] {
            let mut corner: Array2D<bool> = Array2D::filled_with(false, y_max, x_max);
            for x in 0..x_max {
                for y in 0..y_max {
                    corner[(y,x)] = grid[(((y_s as i32 + dy*y as i32 + y_max as i32) % y_max as i32) as usize, ((x_s as i32 + dx*x as i32 + x_max as i32) % x_max as i32) as usize)] == '#';
                }
            }
            let total_corner = reached(corner, steps);
            totals[corner_index] = total_corner - (steps+1) as u64;
            corner_index += 1;
            total += total_corner;
            break;
        }
        break;
    }
    total -= 4_u64*(steps as u64+1)/2;

    println!("{total}");
    println!("{:?}", totals);
    calc_directly(&grid, steps as usize)
}

fn reached(walls: Array2D<bool>, steps: u32) -> u64 {
    println!("Walls:");
    print_grid_bool(&walls);
    let y_max = walls.num_rows();
    let x_max = walls.num_columns();
    let mut states : Vec<Array2D<bool>> = Vec::new();
    let mut initial = Array2D::filled_with(false, y_max, x_max);
    initial[(0, 0)] = true;
    states.push(initial);
    let mut totals : Vec<u64> = Vec::new();
    totals.push(1);
    loop {
        let mut total = 0;
        let current = states.last().unwrap();
        let mut next = Array2D::filled_with(false, y_max, x_max);
        for y in 0..y_max {
            for x in 0..x_max {
                let position = PositionI32 { x: x as i32, y: y as i32 };
                next[(y,x)] = !walls[(y,x)] && DIRECTIONS.iter().any(|direction| {
                    let near = position + *direction;
                    0 <= near.x && near.x < x_max as i32 && 0 <= near.y && near.y < y_max as i32 && current[(near.y as usize, near.x as usize)]
                });
                if next[(y,x)] {
                    total += 1;
                }
            }
        }

        println!("After step {}: {total}", states.len());
/*        print_grid_bool(&current);
        println!("{}", "=========================\n".repeat(5));
*/
        states.push(next);
        totals.push(total);
        if states.len() >= 4 && states[states.len() - 1] == states[states.len() - 3]
                             && states[states.len() - 2] == states[states.len() - 4] {
            println!("cycle at step {}", states.len());
            break;
        }
    }
    let mut total = 0_u64;
    let mut remaining = steps as i32;
    let mut grids = 0_u64;
    loop {
        let in_grid = if remaining >= states.len() as i32 {
            let base = totals.len() as i32 - 4;
            totals[(base + (remaining - base) % 2) as usize]
        }
        else {
            totals[remaining as usize]
        };
        total += in_grid * (grids+1);
        grids += 1;
        remaining -= x_max as i32;
        if remaining < 0 {
            break;
        }
    };
    return total;
}

fn calc_directly(grid_original: &Array2D<char>, steps: usize) {
    let x_max_original = grid_original.num_columns();
    let y_max_original = grid_original.num_rows();
    let scale = steps / min(x_max_original, y_max_original) + 1;
    let y_max = y_max_original * (2*scale + 1);
    let x_max = x_max_original * (2*scale + 1);
    let mut grid = Array2D::filled_with('.', y_max, x_max);
    for s in 0..=2*scale {
        for y in 0..y_max_original {
            for x in 0..x_max_original {
                grid[(s * y_max_original + y, s * x_max_original + x)] =
                    if s != scale && grid_original[(y,x)] == 'S' { '.' } else { grid_original[(y, x)] };
            }
        }
    }
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
    let mut totals : [i32; 5] = [0, 0, 0, 0, 0];
    for step in 1..=steps {
        total = 0;
        for i in 0..5 {
            totals[i] = 0
        }
        for y in 0..y_max {
            for x in 0..x_max {
                let position = PositionI32 { x: x as i32, y: y as i32 };
                next[(y,x)] = grid[(y,x)] != '#' && DIRECTIONS.iter().any(|direction| {
                    let near = position + *direction;
                    0 <= near.x && near.x < x_max as i32 && 0 <= near.y && near.y < y_max as i32 && current[(near.y as usize, near.x as usize)]
                });
                if next[(y,x)] {
                    total += 1;
                    if x == x_max/2 || y == y_max/2 {
                        totals[4] += 1;
                    }
                    else if x < x_max/2 {
                        if y < y_max/2 {
                            totals[0] += 1;
                        }
                        else {
                            totals[1] += 1;
                        }
                    }
                    else {
                        if y < y_max/2 {
                            totals[2] += 1;
                        }
                        else {
                            totals[3] += 1;
                        }
                    }
                }
            }
        }
        let temp = current;
        current = next;
        next = temp;
        println!("After step {step}: {total}")
    }
    println!("{total}");
    println!("{:?}", totals)
}

#[allow(dead_code)]
fn print_grid_bool(grid: &Array2D<bool>) {
    grid.rows_iter().for_each(|line| {
        println!("{}", line.map(|b| if *b { 'x' } else { '.' }).collect::<String>())
    });
}

/*

01234
1234
234
34
4

*/

//620921593253159