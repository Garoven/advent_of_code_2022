#[cfg(test)]
mod tests;

struct Sequence {
    amount: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Sequence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().filter_map(|w| w.parse::<usize>().ok());
        let sequence = Self {
            amount: split.next().expect("Invalid input"),
            from: split.next().expect("Invalid input"),
            to: split.next().expect("Invalid input"),
        };

        Ok(sequence)
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Sequence>) {
    let mut lines = input.lines().rev();

    let mut sequences: Vec<Sequence> = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let sequence: Sequence = std::str::FromStr::from_str(line).expect("Invalid input");
        sequences.push(sequence);
    }
    sequences.reverse();

    let amount_of_stacks = lines.next().unwrap().split_whitespace().count();
    let mut magazine: Vec<Vec<char>> = vec![vec![]; amount_of_stacks];
    for line in lines {
        let mut crates = line
            .chars()
            .enumerate()
            .filter(|(i, _)| *i == 1 || (*i != 0 && (*i - 1) % 4 == 0))
            .map(|(_, c)| c);

        for stack in &mut magazine {
            let c = crates.next().unwrap();
            if !c.is_whitespace() {
                stack.push(c);
            }
        }
    }

    (magazine, sequences)
}

fn part_one(mut magazine: Vec<Vec<char>>, sequences: &[Sequence]) -> String {
    for sequence in sequences {
        let Sequence { amount, from, to } = sequence;
        for _ in 0..*amount {
            let moved_crate = magazine[*from - 1].pop().unwrap();
            magazine[*to - 1].push(moved_crate)
        }
    }

    magazine
        .into_iter()
        .flat_map(|s| s.last().copied())
        .collect()
}

fn part_two(mut magazine: Vec<Vec<char>>, sequences: &[Sequence]) -> String {
    for sequence in sequences {
        let Sequence { amount, from, to } = sequence;
        let stack_len = magazine[*from - 1].len();
        let mut moved_crate: Vec<char> = magazine[*from - 1].drain(stack_len - amount..).collect();

        magazine[*to - 1].append(&mut moved_crate);
    }

    magazine
        .into_iter()
        .flat_map(|s| s.last().copied())
        .collect()
}

pub fn combined(input: &str) {
    let (magazine, sequences) = parse_input(input);
    let part_one_result = part_one(magazine.clone(), &sequences);
    let part_two_result = part_two(magazine, &sequences);

    println!("Part one: {part_one_result}\nPart two: {part_two_result}");
}
