use std::fs;

#[derive(Debug)]
struct Number {
    value: u32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

impl Number {
    fn check_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
        symbols
            .into_iter()
            .filter(|symbol| symbol.y >= self.y.saturating_sub(1) && symbol.y <= self.y + 1)
            .find(|symbol| symbol.x >= self.x_start.saturating_sub(1) && symbol.x <= self.x_end + 1)
            .is_some()
    }
}

impl Symbol {
    fn check_power(&self, numbers: &Vec<Number>) -> u32 {
        let numbers: Vec<&Number> = numbers
            .into_iter()
            .filter(|number| {
                number.y >= self.y.saturating_sub(1)
                    && number.y <= self.y + 1
                    && self.x >= number.x_start.saturating_sub(1)
                    && self.x <= number.x_end + 1
            })
            .collect();

        if numbers.len() == 2 {
            numbers.into_iter().map(|number| number.value).product()
        } else {
            0
        }
    }
}

fn part1(input: &String) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    for (y, line) in input.lines().into_iter().enumerate() {
        let mut n = String::new();

        for (x, char) in line.chars().into_iter().enumerate() {
            if char.is_digit(10) {
                n.push(char);
                if x == line.len() - 1 {
                    numbers.push(Number {
                        value: n.trim().parse().unwrap(),
                        y,
                        x_start: x - n.trim().len() + 1,
                        x_end: x,
                    });
                }
            } else {
                if char != '.' {
                    symbols.push(Symbol { x, y });
                }

                if !n.is_empty() {
                    numbers.push(Number {
                        value: n.trim().parse().unwrap(),
                        y,
                        x_start: x - n.trim().len(),
                        x_end: x - 1,
                    });
                }

                n = String::new();
            }
        }
    }

    let result: u32 = numbers
        .into_iter()
        .filter(|n| n.check_adjacent(&symbols))
        .map(|n| n.value)
        .sum();

    println!("Part 1 Answer: {result}")
}

fn part2(input: &String) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    for (y, line) in input.lines().into_iter().enumerate() {
        let mut n = String::new();

        for (x, char) in line.chars().into_iter().enumerate() {
            if char.is_numeric() {
                n.push(char);
                if x == line.len() - 1 {
                    numbers.push(Number {
                        value: n.trim().parse().unwrap(),
                        y,
                        x_start: x - n.trim().len() + 1,
                        x_end: x,
                    });
                }
            } else {
                if char == '*' {
                    symbols.push(Symbol { x, y });
                }

                if !n.is_empty() {
                    numbers.push(Number {
                        value: n.trim().parse().unwrap(),
                        x_start: x - n.trim().len(),
                        y,
                        x_end: x - 1,
                    });
                }

                n = String::new();
            }
        }
    }

    let result: u32 = symbols.into_iter().map(|s| s.check_power(&numbers)).sum();

    println!("Part 2 Answer: {result}")
}

fn main() {
    let input = fs::read_to_string("./data/3.txt").unwrap();

    part1(&input);
    part2(&input);
}
