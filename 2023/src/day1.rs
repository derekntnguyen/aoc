use std::fs;

fn part1() {
    let file = fs::read_to_string("./data/1.txt").unwrap();

    let nums: Vec<u32> = file
        .lines()
        .map(|line| {
            let mut digits = String::from("");

            line.chars().for_each(|char| {
                if char.is_numeric() {
                    if digits.len() == 0 {
                        digits.push(char);
                    }
                    digits.push(char);
                }
            });
            format!(
                "{}{}",
                digits.chars().nth(0).unwrap(),
                digits.chars().nth(digits.len() - 1).unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .collect();

    println!("Part 1 Answer: {}", nums.iter().sum::<u32>());
}

fn part2() {
    let file = fs::read_to_string("./data/1.txt").unwrap();

    let nums: Vec<u32> = file
        .lines()
        .map(|line| {
            let clean_line: String = (0..line.len())
                .filter_map(|i| {
                    let substring = &line[i..];
                    let cleaned_line = if substring.starts_with("one") {
                        '1'
                    } else if substring.starts_with("two") {
                        '2'
                    } else if substring.starts_with("three") {
                        '3'
                    } else if substring.starts_with("four") {
                        '4'
                    } else if substring.starts_with("five") {
                        '5'
                    } else if substring.starts_with("six") {
                        '6'
                    } else if substring.starts_with("seven") {
                        '7'
                    } else if substring.starts_with("eight") {
                        '8'
                    } else if substring.starts_with("nine") {
                        '9'
                    } else {
                        substring.chars().next().unwrap()
                    };

                    Some(cleaned_line)
                })
                .collect::<String>();

            let mut digits = String::from("");

            clean_line.chars().for_each(|char| {
                if char.is_numeric() {
                    if digits.len() == 0 {
                        digits.push(char);
                    }
                    digits.push(char);
                }
            });
            format!(
                "{}{}",
                digits.chars().nth(0).unwrap(),
                digits.chars().nth(digits.len() - 1).unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .collect();

    println!("Part 2 Answer: {}", nums.iter().sum::<u32>());
}

fn main() {
    part1();
    part2();
}
