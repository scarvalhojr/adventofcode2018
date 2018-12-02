extern crate day03;

use day03::{part1, part2, Claim};

fn parse(input: &[&str]) -> Vec<Claim> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

#[test]
fn examples_part1() {
    let input = [
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
    ];
    assert_eq!(part1(&parse(&input)), 4);
}

#[test]
fn part1_double_overlaps() {
    let input = [
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
        "#4 @ 4,4: 2x2",
    ];
    assert_eq!(part1(&parse(&input)), 7);
}

#[test]
fn part1_multiple_overlaps() {
    let input = [
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
        "#4 @ 4,4: 2x2",
        "#5 @ 3,3: 2x2",
    ];
    assert_eq!(part1(&parse(&input)), 7);
}

#[test]
fn examples_part2() {
    let input = [
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
    ];
    assert_eq!(part2(&parse(&input)), 3);
}
