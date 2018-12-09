extern crate day09;

use day09::{part1, MarbleGame};

#[test]
fn examples_part1() {
    assert_eq!(part1(&mut MarbleGame::new(9), 25), 32);
    assert_eq!(part1(&mut MarbleGame::new(10), 1618), 8317);
    assert_eq!(part1(&mut MarbleGame::new(13), 7999), 146373);
    assert_eq!(part1(&mut MarbleGame::new(17), 1104), 2764);
    assert_eq!(part1(&mut MarbleGame::new(21), 6111), 54718);
    assert_eq!(part1(&mut MarbleGame::new(30), 5807), 37305);
}
