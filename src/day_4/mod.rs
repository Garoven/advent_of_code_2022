#[cfg(test)]
mod tests;

fn parse_input(
    input: &str,
) -> Vec<(
    std::ops::RangeInclusive<usize>,
    std::ops::RangeInclusive<usize>,
)> {
    let mut list = Vec::new();
    for line in input.lines() {
        let (first_range, second_range) = line.split_once(',').expect("Invalid line in input");
        let first_range = first_range
            .split_once('-')
            .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
            .map(|(s, e)| s..=e)
            .expect("Invalid input");
        let second_range = second_range
            .split_once('-')
            .and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
            .map(|(s, e)| s..=e)
            .expect("Invalid input");

        list.push((first_range, second_range));
    }

    list
}

fn part_one(
    list: &[(
        std::ops::RangeInclusive<usize>,
        std::ops::RangeInclusive<usize>,
    )],
) -> usize {
    let mut pairs_count = 0;
    for (first_range, second_range) in list {
        let is_second_in_first =
            first_range.start() <= second_range.start() && second_range.end() <= first_range.end();
        let is_first_in_second =
            second_range.start() <= first_range.start() && first_range.end() <= second_range.end();
        if is_second_in_first || is_first_in_second {
            pairs_count += 1;
        }
    }

    pairs_count
}

fn part_two(
    list: &[(
        std::ops::RangeInclusive<usize>,
        std::ops::RangeInclusive<usize>,
    )],
) -> usize {
    let mut pairs_count = 0;
    for (first_range, second_range) in list {
        if first_range.contains(second_range.start())
            || first_range.contains(second_range.end())
            || second_range.contains(first_range.start())
            || second_range.contains(first_range.end())
        {
            pairs_count += 1;
        }
    }

    pairs_count
}

pub fn combined(input: &str) {
    let list = parse_input(input);
    let part_one_result = part_one(&list);
    let part_two_result = part_two(&list);

    println!("Part one: {part_one_result}\nPart two: {part_two_result}");
}
