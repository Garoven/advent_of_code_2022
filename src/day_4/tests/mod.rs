use super::*;

static INPUT: &str = include_str!("input");

#[test]
fn part_one_test() {
    let list = parse_input(INPUT);
    let result = part_one(&list);
    let expected = 2;

    assert_eq!(expected, result);
}

#[test]
fn part_two_test() {
    let list = parse_input(INPUT);
    let result = part_two(&list);
    let expected = 4;

    assert_eq!(expected, result);
}
