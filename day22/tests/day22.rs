use day22::Cave;

#[test]
fn example_part1() {
    let mut cave = Cave::new(510, 10, 10);
    assert_eq!(cave.total_risk(), 114);
    assert_eq!(cave.min_distance(), 45);
}
