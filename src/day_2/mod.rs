#[cfg(test)]
mod tests;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;

const LOSS: usize = 0;
const DRAW: usize = 3;
const WIN: usize = 6;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let mut list: Vec<(&str, &str)> = Vec::new();
    for line in input.lines() {
        let pair = line.split_once(' ').expect("Invalid line in input");
        list.push(pair)
    }

    list
}

fn part_one(list: &[(&str, &str)]) -> usize {
    let mut points: usize = 0;
    for (left, right) in list {
        match *left {
            "A" => match *right {
                "X" => points += DRAW + ROCK,
                "Y" => points += WIN + PAPER,
                "Z" => points += LOSS + SCISSORS,
                _ => panic!("Invalid letter"),
            },
            "B" => match *right {
                "X" => points += LOSS + ROCK,
                "Y" => points += DRAW + PAPER,
                "Z" => points += WIN + SCISSORS,
                _ => panic!("Invalid letter"),
            },
            "C" => match *right {
                "X" => points += WIN + ROCK,
                "Y" => points += LOSS + PAPER,
                "Z" => points += DRAW + SCISSORS,
                _ => panic!("Invalid letter"),
            },
            _ => panic!("Invalid letter"),
        };
    }

    points
}

fn part_two(list: &[(&str, &str)]) -> usize {
    let mut points: usize = 0;
    for (left, right) in list {
        match *left {
            "A" => match *right {
                "X" => points += LOSS + SCISSORS,
                "Y" => points += DRAW + ROCK,
                "Z" => points += WIN + PAPER,
                _ => panic!("Invalid letter"),
            },
            "B" => match *right {
                "X" => points += LOSS + ROCK,
                "Y" => points += DRAW + PAPER,
                "Z" => points += WIN + SCISSORS,
                _ => panic!("Invalid letter"),
            },
            "C" => match *right {
                "X" => points += LOSS + PAPER,
                "Y" => points += DRAW + SCISSORS,
                "Z" => points += WIN + ROCK,
                _ => panic!("Invalid letter"),
            },
            _ => panic!("Invalid letter"),
        };
    }

    points
}

pub fn combined(input: &str) -> (crate::Display, crate::Display) {
    let list = parse_input(input);

    (Box::new(part_one(&list)), Box::new(part_two(&list)))
}
