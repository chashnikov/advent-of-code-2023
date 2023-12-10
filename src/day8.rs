use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("8-full.txt");
    let mut route : Vec<u32> = Vec::new();
    let mut left : HashMap<String, String> = HashMap::new();
    let mut right : HashMap<String, String> = HashMap::new();
    let regex = Regex::new(r"([0-9A-Z]+) *= *\(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
    content.lines().for_each(|line| {
        if let Some(caps) = regex.captures(line) {
            left.insert(String::from(&caps[1]), String::from(&caps[2]));
            right.insert(String::from(&caps[1]), String::from(&caps[3]));
        }
        else if !line.trim().is_empty() {
            route = line.chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();
        }
    });
    let nodes : Vec<String> = left.keys().sorted_by_key(|k| k.chars().last().unwrap()).map(|k| k.clone()).collect();
    let indexes: HashMap<String, u32> = HashMap::from_iter(
        nodes.iter().enumerate().map(|(i, v)| (v.clone(), i as u32))
    );
    let mut next_index: Vec<u32> = Vec::new();
    for node in &nodes {
        next_index.push(indexes[&left[node]]);
        next_index.push(indexes[&right[node]]);
    }

    let mut answer : u64 = 0;
    let initial_count = nodes.iter().find_position(|k| k.chars().last().unwrap() != 'A').unwrap().0;
    let mut current : Vec<u32> = (0..initial_count).map(|i| i as u32).collect();
    let target = nodes.iter().find_position(|k| k.chars().last().unwrap() == 'Z').unwrap().0 as u32;
    let mut index = 0;
    while !current.iter().all(|k| *k >= target) {
        let direction = route[index];
        for i in 0 .. current.len() {
            current[i] = next_index[(2*current[i] + direction) as usize]
        }
        index = (index + 1) % route.len();
        answer += 1;
        if answer % 10000000 == 0 {
            println!("{answer}")
        }
    }
    println!("found!!!");
    println!("{answer}")
}