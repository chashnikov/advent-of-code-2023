use std::cmp::{max, min};
use std::collections::HashMap;
use array2d::Array2D;
use itertools::Itertools;
use regex::Regex;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("22-full.txt");
    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    let bricks : Vec<Brick> = content.lines().map(|line| {
        let ints = re.captures(line).unwrap().iter().skip(1).map(|s|
            s.unwrap().as_str().parse::<u32>().unwrap()
        ).collect_vec();
        (Position3D { x: ints[0], y: ints[1], z: ints[2]},
         Position3D { x: ints[3], y: ints[4], z: ints[5]})
    }).sorted_by_key(|b| min(b.0.z, b.1.z)).collect_vec();
    println!("Bricks: {:?}", bricks);
    let max_x = bricks.iter().flat_map(|b| vec![b.0.x, b.1.x]).max().unwrap();
    let max_y = bricks.iter().flat_map(|b| vec![b.0.y, b.1.y]).max().unwrap();
    let mut xy_to_z_brick : Array2D<(u32, i32)> = Array2D::filled_with((0, -1), max_x as usize + 1, max_y as usize + 1);
    let mut supported_by : HashMap<usize, Vec<usize>> = HashMap::new();

    bricks.iter().enumerate().for_each(|(i, brick)| {
        let mut under : Vec<(u32, usize)> = Vec::new();
        for x in min(brick.0.x, brick.1.x)..=max(brick.0.x, brick.1.x) {
            for y in min(brick.0.y, brick.1.y)..=max(brick.0.y, brick.1.y) {
                let (prev_z, brick_id) = xy_to_z_brick[(x as usize, y as usize)];
                if brick_id >= 0 {
                    under.push((prev_z, brick_id as usize));
                }
            }
        }
        let z_bottom = under.iter().map(|(z, _)| z).max().map(|m| m+1).unwrap_or(0);
        let lies_on = under.iter().filter(|(z, _)| z + 1 == z_bottom).map(|(_, j)| *j).unique().collect_vec();
        println!("{i} lies on {:?}", lies_on);
        supported_by.insert(i, lies_on);
        let z_top = z_bottom + max(brick.0.z, brick.1.z) - min(brick.0.z, brick.1.z);
        for x in min(brick.0.x, brick.1.x)..=max(brick.0.x, brick.1.x) {
            for y in min(brick.0.y, brick.1.y)..=max(brick.0.y, brick.1.y) {
                xy_to_z_brick[(x as usize, y as usize)] = (z_top, i as i32)
            }
        }
    });
    let answer = (0..bricks.len()).filter(|i| {
        supported_by.iter().all(|(_, ids)| *ids != vec![*i])
    }).count();
    println!("{answer}");
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position3D {
    x: u32,
    y: u32,
    z: u32
}

type Brick = (Position3D, Position3D);