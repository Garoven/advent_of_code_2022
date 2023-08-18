#[cfg(test)]
mod tests;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_one(map: &[(usize, char)], list: &[&str]) -> usize {
    let mut points: usize = 0;
    for rustsack in list {
        let (first_half, second_half) = rustsack.split_at(rustsack.len() / 2);
        let item = first_half
            .chars()
            .find(|c| second_half.contains(*c))
            .expect("Invalid line in input");
        for (v, c) in map {
            if *c == item {
                points += v;
                break;
            }
        }
    }

    points
}

fn part_two(map: &[(usize, char)], list: &[&str]) -> usize {
    let mut points = 0;
    for rustsacks in list.chunks(3) {
        let item = rustsacks[0]
            .chars()
            .find(|c| rustsacks[1].contains(*c) && rustsacks[2].contains(*c))
            .expect("Invalid line in input");
        for (v, c) in map {
            if *c == item {
                points += v;
                break;
            }
        }
    }

    points
}

pub fn combined(input: &str) -> (crate::Display, crate::Display) {
    let map: Vec<(usize, char)> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(v, c)| (v + 1, c))
        .collect();
    let list = parse_input(input);

    (
        Box::new(part_one(&map, &list)),
        Box::new(part_two(&map, &list)),
    )
}
