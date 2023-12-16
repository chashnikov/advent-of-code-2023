use array2d::Array2D;
use itertools::Itertools;
use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("12-full.txt");
    let answer : u64 = content.lines().map(|line| number_of_arrangements(line)).sum();
    println!("{answer}")
}

fn number_of_arrangements(line: &str) -> u64 {
    let (pattern_original, numbers_string) = line.split_once(' ').unwrap();
    let pattern = ".".to_owned() + pattern_original;
    let numbers : Vec<u64> = numbers_string.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
    let mut result: Array2D<u64> = Array2D::filled_with(0, pattern.len(), 1 + numbers.len());
    let pattern_chars : Vec<char> = pattern.chars().collect_vec();
    result[(0, 0)] = 1;
    for p in 1..pattern.len() {
        for n in 0..=numbers.len() {
            let mut arrangements = 0;
            if n > 0 {
                let num = numbers[n-1] as usize;
                if p >= num && pattern[p - num + 1..=p].chars().all(|c| c != '.') && *pattern_chars.get(p - num).unwrap() != '#' {
                    if n == 1 {
                        arrangements += result[(p - num, n - 1)];
                    }
                    else if p >= num+1 {
                        arrangements += result[(p - num-1, n - 1)];
                    }
                }
            }
            if *pattern_chars.get(p).unwrap() != '#' {
                arrangements += result[(p - 1, n)];
            }
            result[(p, n)] = arrangements;
        }
    }
    let total = result[(pattern.len() - 1, numbers.len())];
    println!("{}: {}", line, total);
    return total
}