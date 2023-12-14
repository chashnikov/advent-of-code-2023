use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::read_to_string;
use array2d::Array2D;

pub fn solve() {
    let content = read_to_string("8-full.txt");
    let mut route : Vec<usize> = Vec::new();
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
    let indexes: HashMap<String, usize> = HashMap::from_iter(
        nodes.iter().enumerate().map(|(i, v)| (v.clone(), i))
    );
    let mut next_index: Vec<usize> = Vec::new();
    for node in &nodes {
        next_index.push(indexes[&left[node]]);
        next_index.push(indexes[&right[node]]);
    }

    let answer : u64 = 0;
    let initial_count = nodes.iter().find_position(|k| k.chars().last().unwrap() != 'A').unwrap().0;
    let target = nodes.iter().find_position(|k| k.chars().last().unwrap() == 'Z').unwrap().0;
    let cycles : Vec<Cycle> = (0..initial_count).map(|i| find_cycle(i, &route, &next_index, target)).collect();
    let start = cycles.iter().map(|c| c.start).max().unwrap();
    println!("start: {start}");
    let rems = cycles.iter().map(|c| (c.targets.first().unwrap() - start).to_string()).join(", ");
    let mods = cycles.iter().map(|c| c.len.to_string()).join(", ");
    println!("ChineseRemainder[{{{rems}}}, {{{mods}}}]");

    let mut current : Vec<usize> = (0..initial_count).collect();
    let mut index = 0;
    for step in 0..(start+cycles.iter().map(|c| c.len).max().unwrap()) {
        if step <= start || cycles.iter().any(|c| c.len + c.start == step || *c.targets.first().unwrap() == step) {
            let locations : String = current.iter().map(|n| nodes[*n].clone()).join(", ");
            println!("Step {step}: {locations}")
        }
        let direction = route[index];
        for i in 0..current.len() {
            current[i] = next_index[2*current[i] + direction];
        }
        index = (index + 1) % route.len();
    }

    println!("{answer}")
}

fn find_cycle(initial: usize, route: &Vec<usize>, next_index: &Vec<usize>, first_target: usize) -> Cycle {
    let mut current = initial;
    let mut index = 0;
    let inf = 1_000_000_000;
    let mut steps : Array2D<u64> = Array2D::filled_with(inf, next_index.len() / 2, route.len());
    let mut step : u64 = 0;
    let mut targets : Vec<u64> = Vec::new();
    while steps[(current, index)] == inf {
        steps[(current, index)] = step;
        if current >= first_target {
            targets.push(step);
        }
        let direction = route[index];
        current = next_index[2*current + direction];
        index = (index + 1) % route.len();
        step += 1;
    }
    let start = steps[(current, index)];
    Cycle {
        start,
        len: step - start,
        targets: targets.iter().filter(|s| **s >= start).map(|s| *s).collect()
    }
}

struct Cycle {
    start: u64,
    len: u64,
    targets: Vec<u64>,
}