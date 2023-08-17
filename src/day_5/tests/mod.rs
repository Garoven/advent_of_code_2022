use super::*;

const INPUT: &str = include_str!("./input");

#[test]
fn part_one_test() {
    let (magazine, sequence) = parse_input(INPUT);
    let result = part_one(magazine, &sequence);
    let expect = "CMZ";

    assert_eq!(expect, result.as_str())
}

#[test]
fn part_two_test() {
    let (magazine, sequence) = parse_input(INPUT);
    let result = part_two(magazine, &sequence);
    let expect = "MCD";

    assert_eq!(expect, result.as_str())
}
