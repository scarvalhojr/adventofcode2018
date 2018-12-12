use day11::FuelGrid;

#[test]
fn examples_part1() {
    assert_eq!(FuelGrid::new(5, 8).get_power_level(3, 5), 4);
    assert_eq!(FuelGrid::new(122, 57).get_power_level(122, 79), -5);
    assert_eq!(FuelGrid::new(217, 39).get_power_level(217, 196), 0);
    assert_eq!(FuelGrid::new(153, 71).get_power_level(101, 153), 4);

    assert_eq!(FuelGrid::new(300, 18).max_square_size(3), (33, 45, 29));
    assert_eq!(FuelGrid::new(300, 42).max_square_size(3), (21, 61, 30));
}

#[test]
fn no_panic_invalid_coord() {
    assert_eq!(FuelGrid::new(0, 0).get_power_level(0, 0), 0);
    assert_eq!(FuelGrid::new(1, 0).get_power_level(0, 0), 0);
    assert_eq!(FuelGrid::new(1, 0).get_power_level(2, 2), 0);
}

#[test]
fn examples_part2() {
    assert_eq!(FuelGrid::new(300, 18).max_square(), (90, 269, 16, 113));
    assert_eq!(FuelGrid::new(300, 42).max_square(), (232, 251, 12, 119));
}
