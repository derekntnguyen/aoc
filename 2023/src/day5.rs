use std::{fs, ops::Range, str::FromStr};

#[derive(Debug)]
struct ResourceMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl FromStr for ResourceMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mappings = s
            .split_once("map:\n")
            .unwrap()
            .1
            .split("\n")
            .map(|line| {
                let markers: Vec<u64> = line
                    .split(" ")
                    .map(|marker| marker.parse::<u64>().unwrap())
                    .collect();
                (
                    markers[1]..markers[1] + markers[2],
                    markers[0]..markers[0] + markers[2],
                )
            })
            .collect::<Vec<_>>();

        Ok(ResourceMap { mappings })
    }
}

impl ResourceMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;
        destination_range.start + offset
    }
}

fn part1() {
    let input = fs::read_to_string("./data/5.txt").unwrap();

    let maps = input.split("\n\n").collect::<Vec<_>>();

    let seeds = maps[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("{:?}", seeds);

    let parsed_maps = maps
        .iter()
        .skip(1)
        .map(|map| map.parse::<ResourceMap>().unwrap())
        .collect::<Vec<ResourceMap>>();

    let answer: u64 = seeds
        .iter()
        .map(|seed| {
            parsed_maps
                .iter()
                .fold(*seed, |acc, map| map.translate(acc))
        })
        .collect::<Vec<u64>>()
        .iter()
        .min()
        .unwrap()
        .to_owned();

    println!("Part 1 Answer: {}", answer);
}
fn part2() {
    let input = fs::read_to_string("./data/5.txt").unwrap();

    let maps = input.split("\n\n").collect::<Vec<_>>();

    let seeds = maps[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seeds: Vec<u64> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]).collect::<Vec<u64>>())
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect::<Vec<u64>>();

    let parsed_maps = maps
        .iter()
        .skip(1)
        .map(|map| map.parse::<ResourceMap>().unwrap())
        .collect::<Vec<ResourceMap>>();

    let answer: u64 = seeds
        .iter()
        .map(|seed| {
            parsed_maps
                .iter()
                .fold(*seed, |acc, map| map.translate(acc))
        })
        .collect::<Vec<u64>>()
        .to_owned()
        .iter()
        .min()
        .unwrap()
        .to_owned();

    println!("Part 2 Answer: {}", answer);
}

fn main() {
    part1();
    part2();
}
