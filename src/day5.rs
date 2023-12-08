use std::cmp::{max, min};
use std::{fmt, fs};
use std::fmt::{format, Formatter, write};
use std::iter::Map;
use std::path::Path;
use regex::Regex;

pub fn day5() {
    let content = fs::read_to_string(Path::new("5-full.txt")).expect("input must exist");
    let mut seeds : Vec<Range> = Vec::new();
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut current = Mapping { ranges: Vec::new() };
    let range_regex = Regex::new(r" *(\d+) +(\d+) +(\d+)").unwrap();
    content.lines().for_each(|line| {
        if line.starts_with("seeds:") {
            let seed_data : Vec<i64> = line[6..].split(" ")
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect();
            for i in 0..seed_data.len()/2 {
                seeds.push(Range {
                    start: seed_data[2*i],
                    end: seed_data[2*i] + seed_data[2*i+1]
                })
            }
        }
        else if line.contains("map:") {
            if !current.ranges.is_empty() {
                current.ranges.sort_by_key(|r| r.source.start);
                mappings.push(current.clone());
                current = Mapping { ranges: Vec::new() }
            }
        }
        else if !line.trim().is_empty() {
            let range = range_regex.captures(line).unwrap();
            let dest_start = range[1].parse().unwrap();
            let source_start = range[2].parse().unwrap();
            let len : i64 = range[3].parse().unwrap();
            current.ranges.push(RangeMapping {
                source: Range { start: source_start, end: source_start + len },
                dest: Range { start: dest_start, end: dest_start + len },
            });
        }
    });
    current.ranges.sort_by_key(|r| r.source.start);
    mappings.push(current);

    let answer : i64 =
        seeds.iter().flat_map(|seed| {
            mappings.iter().fold(Vec::from([seed.clone()]), |ranges, m | {
                ranges.iter().flat_map(|range| map_range(m, &range)).collect()
            })
        }).map(|range| range.start).min().unwrap();
    println!("{}", answer)
}

#[derive(Clone)]
struct Mapping {
    ranges: Vec<RangeMapping>
}

#[derive(Clone)]
struct RangeMapping {
    source: Range,
    dest: Range,
}

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64,
}

fn map_num(mapping: &Mapping, source: i64) -> i64 {
    let range = mapping.ranges.iter().find(|r| {
       r.source.start <= source && source < r.source.end
    });
    match range {
        None => source,
        Some(range) => range.dest.start + (source - range.source.start)
    }
}

fn map_range(mapping: &Mapping, source: &Range) -> Vec<Range> {
    let mut dest : Vec<Range> = Vec::new();
    if mapping.ranges.is_empty() { return dest }
    let first_range_start = mapping.ranges.first().unwrap().source.start;
    if source.start < first_range_start {
        dest.push(Range { start: source.start, end: min(source.end, first_range_start) })
    }

    for range_mapping in &mapping.ranges {
        let source = intersection(&source, &range_mapping.source);
        if !is_empty(&source) {
            let dest_start = range_mapping.dest.start + (source.start - range_mapping.source.start);
            dest.push(Range {
                start: dest_start,
                end: dest_start + (source.end - source.start)
            })
        }
    }

    let last_range_end = mapping.ranges.last().unwrap().source.end;
    if last_range_end < source.end {
        dest.push(Range { start: max(source.start, last_range_end), end: source.end})
    }
    println!("{} converted to {}", source.to_string(), dest.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", "));
    return dest
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

fn intersection(r1: &Range, r2: &Range) -> Range {
    Range {
        start: max(r1.start, r2.start),
        end: min(r1.end, r2.end)
    }
}

fn is_empty(r: &Range) -> bool { r.start >= r.end }