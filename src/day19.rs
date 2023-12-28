use std::cmp::{max, min};
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::day19::Comparison::{GREATER, LESS};
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("19-full.txt");
    let mut first_part = true;
    let mut workflows : HashMap<String, Workflow> = HashMap::new();
    let mut parts : Vec<Part> = Vec::new();
    content.lines().for_each(|line| {
        if line.is_empty() {
            first_part = false;
        }
        else if first_part {
            let workflow = parse_workflow(line);
            workflows.insert(workflow.name.clone(), workflow);
        }
        else {
            parts.push(parse_part(line));
        }
    });

    let mut total_accepted = 0_u64;
    let mut queue : Vec<(Ranges, String)> = Vec::new();
    queue.push(([(1,4000); 4], String::from("in")));
    while !queue.is_empty() {
        let (ranges, workflow) = queue.pop().unwrap();
        let workflow = workflows.get(&workflow).expect("unknown workflow");
        let mut current = Some(ranges);
        for rule in &workflow.rules {
            let (accepted, rejected) = split_range(&current.unwrap(), &rule);
            if let Some(accepted) = accepted {
                if rule.destination == "A" {
                    total_accepted += number_of_elements(&accepted)
                }
                else if rule.destination != "R" {
                    queue.push((accepted, rule.destination.clone()));
                }
            }
            current = rejected;
            if current == None {
                break;
            }
        }
        if let Some(accepted) = current {
            if workflow.last == "A" {
                total_accepted += number_of_elements(&accepted)
            }
            else if workflow.last != "R" {
                queue.push((accepted, workflow.last.clone()));
            }
        }
    }

    println!("{total_accepted}")
}

fn parse_part(line: &str) -> Part {
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
    let captures = re.captures(line).expect("cannot parse parts");
    let ratings = (1..=4).map(|i| captures[i].parse().unwrap()).collect_vec();
    return [ratings[0], ratings[1], ratings[2], ratings[3]];
}

fn parse_workflow(line: &str) -> Workflow {
    let re = Regex::new(r"([a-z]+)\{(.+),([a-zAR]+)}").unwrap();
    let captures = re.captures(line).expect(format!("cannot parse workflow {}", line).as_str());
    Workflow {
        name: String::from(&captures[1]),
        rules: parse_rules(&captures[2]),
        last: String::from(&captures[3]),
    }
}

fn parse_rules(string: &str) -> Vec<Rule> {
    let re = Regex::new(r"([xmas])([<>=])(\d+):([a-zAR]+)").unwrap();
    string.split(",").map(|rule| {
        let captures = re.captures(rule).unwrap();
        Rule {
            i: PARTS.chars().position(|c| c.to_string() == captures[1]).unwrap(),
            operation: parse_operation(&captures[2]),
            value: captures[3].parse().unwrap(),
            destination: String::from(&captures[4])
        }
    }).collect_vec()
}

fn parse_operation(s: &str) -> Comparison {
    match s {
        "<" => LESS,
        ">" => GREATER,
        _ => panic!("unexpected operation {}", s)
    }
}

fn number_of_elements(ranges: &Ranges) -> u64 {
    ranges.map(|(a, b)| b - a + 1).iter().map(|w| *w as u64 ).product()
}

fn split_range(ranges: &Ranges, rule: &Rule) -> (Option<Ranges>, Option<Ranges>) {
    let (a, b) = ranges[rule.i];
    let accepted = match rule.operation {
        LESS => if a < rule.value { Some((a, min(b, rule.value-1))) } else { None }
        GREATER => if b > rule.value { Some((max(a, rule.value+1), b)) } else { None },
    };
    let rejected = match rule.operation {
        LESS => if b >= rule.value { Some((max(a, rule.value), b)) } else { None }
        GREATER => if a <= rule.value { Some((a, min(b, rule.value))) } else { None },
    };
    let accepted_ranges = accepted.map(|r| {
        let mut new_ranges = ranges.clone();
        new_ranges[rule.i] = r;
        new_ranges
    });
    let rejected_ranges = rejected.map(|r| {
        let mut new_ranges = ranges.clone();
        new_ranges[rule.i] = r;
        new_ranges
    });
    (accepted_ranges, rejected_ranges)
}

const PARTS: &str = "xmas";
type Part = [u32; 4];

type Ranges = [(u32, u32); 4];

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    last: String,
}

enum Comparison {
    LESS, GREATER
}
struct Rule {
    i: usize,
    operation: Comparison,
    value: u32,
    destination: String
}