extern crate day06;

use day06::{part1, part2, Point};

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}

#[test]
fn examples() {
    let input = "1, 1
                 1, 6
                 8, 3
                 3, 4
                 5, 5
                 8, 9";

    let points = parse_input(&input);
    assert_eq!(part1(&points), 17);
    assert_eq!(part2(&points, 32), 16);
}
