use std::convert::identity;
use crate::read_to_string;
use itertools::Itertools;
use crate::day7::Kind::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};


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
    FiveOfAKind, // all five cards have the same label: AAAAA
    FourOfAKind, // four cards have the same label and one card has a different label: AA8AA
    FullHouse, // three cards have the same label, and the remaining two cards share a different label: 23332
    ThreeOfAKind, // three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    TwoPair, // two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    OnePair, // two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    HighCard
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
        1 => FiveOfAKind,
        2 => if max_count == 4 { FourOfAKind } else { FullHouse },
        3 => if max_count == 3 { ThreeOfAKind } else { TwoPair },
        4 => OnePair,
        _ => HighCard
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