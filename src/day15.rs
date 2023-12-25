use crate::read_to_string;

pub fn solve() {
    let content = read_to_string("15-full.txt");
    let answer : u64 = content.split(',').map(|w| hash(w)).sum();
    println!("{answer}");
}

fn hash(s: &str) -> u64 {
    s.bytes().fold(0_u64, |acc, x| (acc + x as u64)*17%256)
}