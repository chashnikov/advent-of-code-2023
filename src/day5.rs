use std::fs;
use std::iter::Map;
use std::path::Path;
use regex::Regex;

pub fn day5() {
    let content = fs::read_to_string(Path::new("5-full.txt")).expect("input must exist");
    let mut seeds : Vec<u64> = Vec::new();
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut current = Mapping { ranges: Vec::new() };
    let range_regex = Regex::new(r" *(\d+) +(\d+) +(\d+)").unwrap();
    content.lines().for_each(|line| {
        if line.starts_with("seeds:") {
            seeds = line[6..].split(" ")
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect()
        }
        else if line.contains("map:") {
            if !current.ranges.is_empty() {
                mappings.push(current.clone());
                current = Mapping { ranges: Vec::new() }
            }
        }
        else if !line.trim().is_empty() {
            let range = range_regex.captures(line).unwrap();
            let dest_start = range[1].parse().unwrap();
            let source_start = range[2].parse().unwrap();
            let len = range[3].parse().unwrap();
            current.ranges.push(Range {
                dest_start,
                source_start,
                len,
            });
        }
    });
    mappings.push(current);

    let answer : u64 =
        seeds.iter().map(|seed| {
            mappings.iter().fold(*seed, |v, m | map_num(m, v))
        }).min().unwrap();
    println!("{}", answer)
}

#[derive(Clone)]
struct Mapping {
    ranges: Vec<Range>
}

#[derive(Clone)]
struct Range {
    source_start: u64,
    dest_start: u64,
    len: u64,
}

fn map_num(mapping: &Mapping, source: u64) -> u64 {
    let range = mapping.ranges.iter().find(|r| {
       r.source_start <= source && source < r.source_start + r.len
    });
    match range {
        None => source,
        Some(range) => range.dest_start + (source - range.source_start)
    }
}