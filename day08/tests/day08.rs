extern crate day08;

use day08::{part1, part2, NodeGraph};

fn parse_input(input: &str) -> Vec<u8> {
    input.split(' ').map(|s| s.parse().unwrap()).collect()
}

#[test]
fn examples() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let values = parse_input(&input);
    let graph = NodeGraph::build_graph(&values);
    assert_eq!(part1(&graph), 138);
    assert_eq!(part2(&graph), 66);
}
