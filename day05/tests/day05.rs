use day05::{reduced_polymer_len, shortest_reduction};

#[test]
fn examples_part1() {
    assert_eq!(reduced_polymer_len("aA"), 0);
    assert_eq!(reduced_polymer_len("abBA"), 0);
    assert_eq!(reduced_polymer_len("abAB"), 4);
    assert_eq!(reduced_polymer_len("aabAAB"), 6);
    assert_eq!(reduced_polymer_len("dabAcCaCBAcCcaDA"), 10);
}

#[test]
fn examples_part2() {
    assert_eq!(shortest_reduction("dabAcCaCBAcCcaDA"), 4);
}
