use std::fs;
use std::path::Path;

pub fn solve() {
    let string = fs::read_to_string(Path::new("2-full.txt")).expect("input exists");
    let games : Vec<Game> = string.lines().map(|line| { parse_game(line) }).collect();
    let answer : u64 = games.iter()
        .map(|game| {
            game.bags.iter().map(|bag| bag.red).max().unwrap()*
            game.bags.iter().map(|bag| bag.green).max().unwrap()*
            game.bags.iter().map(|bag| bag.blue).max().unwrap()
        })
        .sum();
    println!("{}", answer)
}

struct Game {
    bags: Vec<Bag>
}

struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

fn parse_game(line: &str) -> Game {
    let colon = line.find(':').expect(": must be found");
    Game {
        bags: line[colon+2..].split("; ").map(|bag| { parse_bag(bag) }).collect()
    }
}

fn parse_bag(bag: &str) -> Bag {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    bag.split(", ").for_each(|item| {
        let space = item.find(' ').expect("bag is number with color");
        let number : u64 = item[0..space].parse().unwrap();
        match &item[space+1..] {
            "red" => { red += number }
            "green" => { green += number }
            "blue" => { blue += number }
            _ => panic!("unexpected color")
        }
    });
    Bag { red, green, blue }
}