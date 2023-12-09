use std::convert::identity;
use regex::Regex;
use crate::read_to_string;
use itertools::Itertools;
use crate::day7::Kind::{Five_of_a_kind, Four_of_a_kind, Full_house, High_card, One_pair, Three_of_a_kind, Two_pair};


pub fn solve() {
    let content = read_to_string("7-full.txt");
    let mut hands: Vec<Hand> = content.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let chars : Vec<char> = cards.chars().collect();
        Hand {
            cards: [chars[0], chars[1], chars[2], chars[3], chars[4]],
            bid: bid.parse().unwrap()
        }
    }).collect();
    hands.sort_by(|h1, h2| {
        let k1 = kind(h1.cards);
        let k2 = kind(h2.cards);
        k1.cmp(&k2).reverse().then(
                 h1.cards.iter().map(|c| rank(c))
            .cmp(h2.cards.iter().map(|c| rank(c))))
    });
    let answer : u64 = hands.iter().enumerate().map(|(i, v)| {
        ((i+1) as u64)*v.bid
    }).sum();
    println!("{answer}")
}

struct Hand {
    cards: [char; 5],
    bid: u64,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Kind {
    Five_of_a_kind, // all five cards have the same label: AAAAA
    Four_of_a_kind, // four cards have the same label and one card has a different label: AA8AA
    Full_house, // three cards have the same label, and the remaining two cards share a different label: 23332
    Three_of_a_kind, // three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two_pair, // two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One_pair, // two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High_card
}

fn kind(cards: [char; 5]) -> Kind {
    let mut counts : Vec<(char, usize)> = cards.iter().filter(|c| **c != 'J')
        .counts_by(identity).iter().sorted_by_key(|(_, v)| *v)
        .rev().map(|(ch, count)| (**ch, *count)).collect();
    let jokers = cards.iter().filter(|c| **c == 'J').count();
    let first = if counts.is_empty() { ('J', 0) } else { counts.remove(0) };
    counts.insert(0, (first.0, first.1 + jokers));
    let max_count = counts.first().unwrap().1;
    match counts.len() {
        1 => Five_of_a_kind,
        2 => if max_count == 4 { Four_of_a_kind } else { Full_house },
        3 => if max_count == 3 { Three_of_a_kind } else { Two_pair },
        4 => One_pair,
        _ => High_card
    }
}
fn rank(card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap()
    }
}