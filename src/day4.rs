use std::collections::HashSet;
use std::fs;
use std::path::Path;
use regex::{Captures, Regex};

pub fn day4() {
    let content = fs::read_to_string(Path::new("4-full.txt")).expect("input must exist");
    let re = Regex::new(r"Card +\d+:(.*)\|(.*)").unwrap();
    let matches : Vec<u32> = content.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let score = set_of_words(&cap[1]).intersection(&set_of_words(&cap[2])).count();
        score as u32
    }).collect();

    let mut counts = vec![1; matches.len()];
    for i in 0..matches.len() {
        for j in 1..=matches[i] {
            counts[i+j as usize] += counts[i]
        }
    }
    let answer : u64 = counts.iter().sum();
    println!("{}", answer)
}

fn set_of_words(line: &str) -> HashSet<u64> {
    HashSet::from_iter(line.split(" ").filter(|w| !w.is_empty()).map(|w| w.parse::<u64>().unwrap()))
}