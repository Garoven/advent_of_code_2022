use super::*;

const INPUT: &str = include_str!("./input");

#[test]
fn part_one_test() {
    let result = part_one(INPUT);
    let expected = 7;

    assert_eq!(expected, result)
}

#[test]
fn part_two_test() {
    let result = part_two(INPUT);
    let expected = 19;

    assert_eq!(expected, result)
}
