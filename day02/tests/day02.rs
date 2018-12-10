use day02::{part1, part2};

#[test]
fn examples_part1() {
    let input = [
        "abcdef".to_string(),
        "bababc".to_string(),
        "abbcde".to_string(),
        "abcccd".to_string(),
        "aabcdd".to_string(),
        "abcdee".to_string(),
        "ababab".to_string(),
    ];
    assert_eq!(part1(&input), 12);
}

#[test]
fn examples_part2() {
    let input = [
        "abcde".to_string(),
        "fghij".to_string(),
        "klmno".to_string(),
        "pqrst".to_string(),
        "fguij".to_string(),
        "axcye".to_string(),
        "wvxyz".to_string(),
    ];
    assert_eq!(part2(&input), "fgij");
}
