use std::collections::HashMap;
use regex::Regex;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("8-full.txt");
    let mut route : Vec<u8> = Vec::new();
    let mut left : HashMap<String, String> = HashMap::new();
    let mut right : HashMap<String, String> = HashMap::new();
    let regex = Regex::new(r"([A-Z]+) *= *\(([A-Z]+), ([A-Z]+)\)").unwrap();
    content.lines().for_each(|line| {
        if let Some(caps) = regex.captures(line) {
            left.insert(String::from(&caps[1]), String::from(&caps[2]));
            right.insert(String::from(&caps[1]), String::from(&caps[3]));
        }
        else if !line.trim().is_empty() {
            route = line.chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();
        }
    });
    let mut answer : u32 = 0;
    let mut current = String::from("AAA");
    let mut index = 0;
    while current != "ZZZ" {
        let next  = route[index];
        current = if next == 0 { left[&current].clone() } else { right[&current].clone() };
        index = (index + 1) % route.len();
        answer += 1
    }
    println!("{answer}")
}