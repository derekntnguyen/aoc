use std::collections::HashSet;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Card {
    // number: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    fn check_winning_numbers(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .collect::<Vec<&u32>>()
            .len()
            .try_into()
            .unwrap()
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_split = s.split(":").nth(0).unwrap();
        // let number = first_split.split(" ").nth(1).unwrap().parse().unwrap();

        let second_split = s.split(":").nth(1).unwrap();
        let winning_numbers: HashSet<u32> = second_split
            .trim()
            .split(" | ")
            .nth(0)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let card_numbers: HashSet<u32> = second_split
            .trim()
            .split(" | ")
            .nth(1)
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Card {
            // number,
            winning_numbers,
            card_numbers,
        })
    }
}

fn part1() {
    let input = fs::read_to_string("./data/4.txt").unwrap();
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();

    let answer = cards
        .iter()
        .map(|card| {
            if card.check_winning_numbers() == 0 {
                0
            } else {
                2_u32.pow(card.check_winning_numbers().saturating_sub(1))
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    println!("Part 1 Answer {answer}")
}

fn main() {
    part1();
}
