use crate::{read_to_string, string_to_i64_numbers};

pub(crate) fn solve() {
    let content = read_to_string("9-full.txt");
    let answer : i64 = content.lines().map(|line| find_next(line)).sum();
    println!("{answer}");
}

fn find_next(line: &str) -> i64 {
    let mut numbers = string_to_i64_numbers(line);
    numbers.reverse();
    return find_next_num(&numbers)
}

fn find_next_num(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|i| *i == 0) {
        return 0;
    }
    let mut diffs : Vec<i64> = Vec::new();
    for i in 0 .. numbers.len()-1 {
        diffs.push(numbers[i + 1] - numbers[i]);
    }
    return numbers.last().unwrap() + find_next_num(&diffs);
}