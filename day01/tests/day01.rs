extern crate day01;

use day01::{part1, part2};

#[test]
fn examples_part1() {
    assert_eq!(part1(&[1, -2, 3, 1]), 3);
}

#[test]
fn examples_part2() {
    assert_eq!(part2(&[1, -2, 3, 1]), 2);
    assert_eq!(part2(&[1, -1]), 1);
    assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
}
