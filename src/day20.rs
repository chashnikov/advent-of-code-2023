use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use regex::Regex;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("20-full.txt");
    let re = Regex::new(r"([&%]?)([a-z]+) -> ([a-z, ]+)").unwrap();
    let mut modules : HashMap<String, Module> = HashMap::new();
    let mut inputs : HashMap<String, Vec<String>> = HashMap::new();

    content.lines().for_each(|line| {
        let captures = re.captures(line).expect("cannot parse");
        let kind = &captures[1];
        let name = &captures[2];
        let destinations = captures[3].split(", ").map(|s| String::from(s)).collect_vec();
        for destination in &destinations {
            if let Some(destination_module) = modules.get_mut(destination) {
                if let Module::Conjunction { inputs, destinations: _} = destination_module {
                    inputs.insert(name.to_string(), false);
                }
            }
            else if let Some(inputs) = inputs.get_mut(destination) {
                inputs.push(name.to_string());
            }
            else {
                inputs.insert(destination.clone(), vec![name.to_string()]);
            }
        }
        let module = if name == "broadcaster" {
            Module::Broadcaster { destinations }
        }
        else if kind == "%" {
            Module::FlipFlop {
                on: false,
                destinations,
            }
        }
        else if kind == "&" {
            let inputs = inputs.remove(&name.to_string()).unwrap_or(Vec::new()).iter().map(|name| (name.clone(), false)).collect::<HashMap<String, bool>>();
            Module::Conjunction {
                inputs,
                destinations,
            }
        }
        else {
            panic!("unknown kind for {name}")
        };
        modules.insert(name.to_string(), module);
    });

    println!("Modules");
    for (name, module) in &modules {
        println!("{}: {:?}", name, module)
    }

    let mut total = Pulses { low:0, high: 0};
    let mut analysis = Analysis {
        on: [Vec::new(), Vec::new(), Vec::new(), Vec::new()]
    };
    for count in 1..10000 {
        let pulses = push_button(&mut modules, count, &mut analysis);
        total.low += pulses.low;
        total.high += pulses.high;
    }
    for i in 0..4 {
        println!("[{i}]=1:");
        for s in &analysis.on[i] {
            println!("  {s}")
        }
    }
}

fn push_button(mut modules: &mut HashMap<String, Module>, count: u64, analysis: &mut Analysis) -> Pulses {
    let mut pulses = Pulses { low: 0, high: 0};
    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
    queue.push_back(("BUTTON".to_string(), "broadcaster".to_string(), false));
    while !queue.is_empty() {
        let (source, destination, pulse) = queue.pop_front().unwrap();
        if pulse {
            pulses.high += 1;
        }
        else {
            pulses.low += 1;
        }
        send(source, destination, pulse, &mut modules, &mut queue, count, analysis);
    }
    return pulses;
}

fn send(source: String, destination: String, pulse: bool, modules: &mut HashMap<String, Module>,
        queue: &mut VecDeque<(String, String, bool)>, count: u64, analysis: &mut Analysis) {
    // println!("{source} -{pulse}-> {destination}");
    let module = if let Some(module) = modules.get_mut(&destination) {
        module
    }
    else {
        return;
    };
    match module {
        Module::Broadcaster { destinations } => {
            send_to_destinations(destination, destinations, pulse, queue);
        },
        Module::FlipFlop { on, destinations } => {
            if !pulse {
                let new_value = !*on;
                *on = new_value;
                send_to_destinations(destination, destinations, new_value, queue);
            }
        },
        Module::Conjunction { inputs, destinations} => {
            inputs.insert(source, pulse);
            if destination == "qn" {
                inputs.iter().sorted_by_key(|e| e.0).enumerate().for_each(|(i, (_, value))| {
                    if *value && analysis.on[i].last() != Some(&count) {
                        analysis.on[i].push(count);
                    }
                });
                // println!("{count} clicks: {}", inputs.values().map(|p| if *p { '1' } else { '0' }).collect::<String>())
            }
            let next = !inputs.values().all(|p| *p);
            send_to_destinations(destination, destinations, next, queue);
        }
    }
}

fn send_to_destinations(source: String, destinations: &Vec<String>, pulse: bool, queue: &mut VecDeque<(String, String, bool)>) {
    for destination in destinations {
        queue.push_back((source.clone(), destination.clone(), pulse));
    }
}

#[derive(Debug)]
enum Module {
    Broadcaster {
        destinations: Vec<String>
    },
    FlipFlop {
        on: bool,
        destinations: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, bool>,
        destinations: Vec<String>,
    }
}

struct Pulses {
    low: u64,
    high: u64,
}

struct Analysis {
    on: [Vec<u64>; 4]
}