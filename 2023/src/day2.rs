use std::cmp::max;
use std::fmt::Error;
use std::{fs, str::FromStr};

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn min_draw(&self) -> Draw {
        let mut draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.draws.iter().for_each(|item| {
            draw.red = max(item.red, draw.red);
            draw.green = max(item.green, draw.green);
            draw.blue = max(item.blue, draw.blue);
        });
        draw
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let id = parts
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let draws = parts
            .next()
            .unwrap()
            .split(";")
            .map(|draw| draw.parse::<Draw>().unwrap())
            .collect();

        Ok(Game { id, draws })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn compare_draws(self, other: Draw) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Draw {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };

        s.trim()
            .split(",")
            .map(|ball| ball.trim().parse::<Ball>().unwrap())
            .for_each(|ball| match ball {
                Ball::Red(count) => draw.red += count,
                Ball::Green(count) => draw.green += count,
                Ball::Blue(count) => draw.blue += count,
            });

        Ok(draw)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Ball {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Ball {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let count = parts.next().unwrap().parse::<u32>().unwrap();

        Ok(match parts.next().unwrap() {
            "red" => Ball::Red(count),
            "green" => Ball::Green(count),
            "blue" => Ball::Blue(count),
            _ => panic!("Invalid ball color"),
        })
    }
}

fn part1() {
    let possible_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let nums = fs::read_to_string("./data/2.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.min_draw().compare_draws(possible_draw))
        .map(|game| game.id)
        .collect::<Vec<u32>>();

    println!("Part 1 Answer: {}", nums.iter().sum::<u32>());
}
fn part2() {
    let nums = fs::read_to_string("./data/2.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.min_draw().power())
        .collect::<Vec<u32>>();

    println!("Part 2 Answer: {}", nums.iter().sum::<u32>());
}

fn main() {
    part1();
    part2();
}
