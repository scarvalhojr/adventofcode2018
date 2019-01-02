use day13::Mine;

#[test]
fn example_part1() {
    #[rustfmt::skip]
    let sample = [
        r"/->-\        ",
        r"|   |  /----\",
        r"| /-+--+-\  |",
        r"| | |  | v  |",
        r"\-+-/  \-+--/",
        r"  \------/   ",
    ]
    .join("\n");
    let mut mine: Mine = sample.parse().unwrap();
    assert_eq!(mine.first_crash_pos(), (7, 3));
}

#[test]
fn example_part2() {
    #[rustfmt::skip]
    let sample = [
        r"/>-<\  ",
        r"|   |  ",
        r"| /<+-\",
        r"| | | v",
        r"\>+</ |",
        r"  |   ^",
        r"  \<->/",
    ]
    .join("\n");
    let mut mine: Mine = sample.parse().unwrap();
    assert_eq!(mine.last_cart_pos(), (6, 4));
}
