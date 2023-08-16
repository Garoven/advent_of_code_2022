#[cfg(test)]
mod tests;

fn parse_input(input: &str) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();
    let mut current_elf: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            list.push(current_elf);
            current_elf = 0;
            continue;
        } else {
            current_elf += line.parse::<usize>().expect("Invalid line in input");
        }
    }
    list.push(current_elf);

    list.sort();
    list
}

fn part_one(list: &[usize]) -> usize {
    let len = list.len();
    if len >= 1 {
        list[len - 1]
    } else {
        panic!("Empty imput")
    }
}

fn part_two(list: &[usize]) -> usize {
    let len = list.len();
    if len >= 3 {
        list[len - 3..].iter().sum()
    } else {
        panic!("Empty input")
    }
}

pub fn combined(input: &str) {
    let list = parse_input(input);
    let part_one_result = part_one(&list);
    let part_two_result = part_two(&list);

    println!("Part one: {part_one_result}\nPart two: {part_two_result}");
}
