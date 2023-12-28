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
    let answer: u32 = parts.iter()
        .filter(|part| is_accepted(part, &workflows))
        .map(|part| {
            part.iter().sum::<u32>()
        })
        .sum();
    println!("{answer}")
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in";
    loop {
        let workflow = workflows.get(current).expect(format!("unknown workflow {}", current).as_str());
        let mut found = false;
        for rule in &workflow.rules {
            let next = next_rule(part, rule);
            if let Some(next) = next {
                current = next;
                found = true;
                break;
            }
        }
        if !found {
            current = &workflow.last;
        }
        if current == "A" { return true; }
        if current == "R" { return false; }
    }
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

fn next_rule<'a>(part: &Part, rule: &'a Rule) -> Option<&'a String> {
    let v = part[rule.i];
    let matches = match rule.operation {
        LESS => v < rule.value,
        GREATER => v > rule.value,
    };
    if matches { Some(&rule.destination) } else { None }
}

const PARTS: &str = "xmas";
type Part = [u32; 4];
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