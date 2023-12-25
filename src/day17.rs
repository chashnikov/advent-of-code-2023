use std::hash::Hash;
use std::iter::Iterator;
use array2d::Array2D;
use itertools::Itertools;
use priority_queue::DoublePriorityQueue;
use crate::{Direction, DIRECTIONS, EAST, PositionI32, read_to_string, SOUTH};

const VARIANTS: usize = 4 * 10;

pub fn solve() {
    let content = read_to_string("17-full.txt");
    let grid = Array2D::from_rows(content.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec().as_slice()).expect("correct grid");
    let x_max = grid.num_columns();
    let y_max = grid.num_rows();
    let mut min_cost: Array2D<[u32; VARIANTS]> = Array2D::filled_with([1_000_000_000; VARIANTS], grid.num_rows(), grid.num_columns());
    let mut next : DoublePriorityQueue<(PositionI32, Impulse), u32> = DoublePriorityQueue::new();

    let impulses = (0..=9).flat_map(|r| DIRECTIONS.map(|d| Impulse {
        direction: d,
        remaining: r,
    })).collect_vec();
    assert_eq!(impulses.len(), VARIANTS);

    next.push((PositionI32 { x: 0, y: 1}, Impulse {direction: SOUTH, remaining: 9}), grid[(1,0)]);
    next.push((PositionI32 { x: 1, y: 0}, Impulse {direction: EAST, remaining: 9}), grid[(0,1)]);
    while !next.is_empty() {
        let ((position, impulse), cost) = next.pop_min().unwrap();
        // println!("Reached {} with {}", position, cost);
        let impulse_index = impulses.iter().position(|i| *i == impulse).unwrap();
        let index = (position.y as usize, position.x as usize);
        if cost >= min_cost[index][impulse_index] {
            continue
        }
        min_cost[index][impulse_index] = cost;
        for direction in DIRECTIONS {
            if direction == impulse.direction.opposite() || direction == impulse.direction && impulse.remaining == 0
               || direction != impulse.direction && impulse.remaining > 6 {
                continue
            }
            let remaining = if direction == impulse.direction { impulse.remaining - 1 } else { 9 };
            let new_position = position + direction;
            if new_position.x < 0 || new_position.x >= x_max as i32 || new_position.y < 0 || new_position.y >= y_max as i32 {
                continue
            }
            let new_cost = cost + grid[(new_position.y as usize, new_position.x as usize)];
            let item = (new_position, Impulse { direction, remaining });
            let existing = next.get_priority(&item);
            let update = match existing {
                None => true,
                Some(&old_cost) => old_cost > new_cost
            };
            if update {
                next.push(item, new_cost);
            }
        }
    }
    let answer = min_cost[(y_max-1, x_max-1)].iter().enumerate()
        .filter(|(i, _)| impulses.get(*i).unwrap().remaining <= 6)
        .map(|(_, v)| v)
        .min()
        .unwrap();
    println!("{answer}")
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Impulse {
    direction: Direction,
    remaining: u8,
}
