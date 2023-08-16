use super::*;

static INPUT: &str = include_str!("input");

fn get_map() -> Vec<(usize, char)> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(v, c)| (v + 1, c))
        .collect()
}

#[test]
fn part_one_test() {
    let map = get_map();
    let list = parse_input(INPUT);
    let result = part_one(&map, &list);
    let expected = 157;

    assert_eq!(expected, result);
}

#[test]
fn part_two_test() {
    let map = get_map();
    let list = parse_input(INPUT);
    let result = part_two(&map, &list);
    let expected = 70;

    assert_eq!(expected, result);
}
