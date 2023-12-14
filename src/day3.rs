use std::fs;
use std::path::Path;

pub fn day3() {
    let content = fs::read_to_string(Path::new("3-full.txt")).expect("input must exist");
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Coord> = Vec::new();
    content.lines().enumerate().for_each(|(y, line)| {
        let mut num : u32 = 0;
        let mut num_start : Option<Coord> = None;
        let mut num_end : Option<Coord> = None;
        line.chars().enumerate().for_each(|(x, c)| {
            let coord = Coord { x: x as i32, y: y as i32 };
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Option::from(coord);
                    num = 0;
                }
                num = num*10 + c.to_digit(10).unwrap();
                num_end = Option::from(coord.clone())
            } else {
                if num_start.is_some() {
                    numbers.push(Number {
                        num,
                        left: num_start.unwrap(),
                        right: num_end.unwrap()
                    });
                    num_start = None;
                }
                if c == '*' {
                    gears.push(coord.clone())
                }
            }
        });
        if num_start.is_some() {
            numbers.push(Number {
                num,
                left: num_start.unwrap(),
                right: num_end.unwrap()
            });
        }
    });
    /*
        numbers.iter().for_each(|n| {
           println!("{}: {},{}", n.num, n.left.x, n.left.y)
        });
    */
    let answer : u64 = gears.iter().map(|g| {
        let nums : Vec<&Number> = numbers.iter().filter(|n| {
            n.left.y - 1 <= g.y && g.y <= n.left.y + 1 &&
                n.left.x - 1 <= g.x && g.x <= n.right.x + 1
        }).collect();
        if nums.len() == 2 {
            // println!("{}, {}", nums[0].num, nums[1].num);
            (nums[0].num as u64) * (nums[1].num as u64)
        }
        else {
            0 as u64
        }
    }).sum();
    println!("{}", answer);
}

struct Number {
    num: u32,
    left: Coord,
    right: Coord,
}

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}