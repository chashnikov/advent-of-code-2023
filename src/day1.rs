use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn day1() {
    let string = fs::read_to_string(Path::new("1.txt")).expect("input exists");
    let mut numbers: HashMap<String, u32> = HashMap::new();
    numbers.insert(String::from("one"), 1);
    numbers.insert(String::from("two"), 2);
    numbers.insert(String::from("three"), 3);
    numbers.insert(String::from("four"), 4);
    numbers.insert(String::from("five"), 5);
    numbers.insert(String::from("six"), 6);
    numbers.insert(String::from("seven"), 7);
    numbers.insert(String::from("eight"), 8);
    numbers.insert(String::from("nine"), 9);

    let result: u32 = string
        .lines()
        .map(|line| -> u32 {
            find_first_number(&line, &numbers) * 10 +
                find_last_number(&line, &numbers)
        })
        .sum();
    println!("{}", result)
}

fn find_first_number(line: &str, numbers: &HashMap<String, u32>) -> u32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap()
        }
        for (number, &v) in numbers {
            if line[i..].starts_with(number) {
                return v
            }
        }
    }
    panic!("not found")
}

fn find_last_number(line: &str, numbers: &HashMap<String, u32>) -> u32 {
    let len = line.chars().count();
    for (i, c) in line.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap()
        }
        for (number, &v) in numbers {
            if line[(len - 1 - i)..].starts_with(number) {
                return v
            }
        }
    }
    panic!("not found")
}