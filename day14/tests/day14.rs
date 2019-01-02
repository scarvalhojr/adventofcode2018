use day14::Scoreboard;

#[test]
fn examples_part1() {
    let mut scoreboard: Scoreboard = Default::default();
    assert_eq!(scoreboard.get_score_after(9), "5158916779");
    assert_eq!(scoreboard.get_score_after(5), "0124515891");
    assert_eq!(scoreboard.get_score_after(18), "9251071085");
    assert_eq!(scoreboard.get_score_after(2018), "5941429882");
}

#[test]
fn examples_part2() {
    let mut scoreboard: Scoreboard = Default::default();
    assert_eq!(scoreboard.find_pattern("51589"), 9);
    assert_eq!(scoreboard.find_pattern("01245"), 5);
    assert_eq!(scoreboard.find_pattern("92510"), 18);
    assert_eq!(scoreboard.find_pattern("59414"), 2018);
}
