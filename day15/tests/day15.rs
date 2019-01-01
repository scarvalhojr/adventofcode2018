use day15::{part2, Battle};

#[test]
fn examples() {
    let tests = [
        (
            concat!(
                "#######\n",
                "#.G...#\n",
                "#...EG#\n",
                "#.#.#G#\n",
                "#..G#E#\n",
                "#.....#\n",
                "#######\n",
            ),
            27730,
            4988,
        ),
        (
            concat!(
                "#######\n",
                "#G..#E#\n",
                "#E#E.E#\n",
                "#G.##.#\n",
                "#...#E#\n",
                "#...E.#\n",
                "#######\n",
            ),
            36334,
            29064,
        ),
        (
            concat!(
                "#######\n",
                "#E..EG#\n",
                "#.#G.E#\n",
                "#E.##E#\n",
                "#G..#.#\n",
                "#..E#.#\n",
                "#######\n",
            ),
            39514,
            31284,
        ),
        (
            concat!(
                "#######\n",
                "#E.G#.#\n",
                "#.#G..#\n",
                "#G.#.G#\n",
                "#G..#.#\n",
                "#...E.#\n",
                "#######\n",
            ),
            27755,
            3478,
        ),
        (
            concat!(
                "#######\n",
                "#.E...#\n",
                "#.#..G#\n",
                "#.###.#\n",
                "#E#G#G#\n",
                "#...#G#\n",
                "#######\n",
            ),
            28944,
            6474,
        ),
        (
            concat!(
                "#########\n",
                "#G......#\n",
                "#.E.#...#\n",
                "#..##..G#\n",
                "#...##..#\n",
                "#...#...#\n",
                "#.G...G.#\n",
                "#.....G.#\n",
                "#########\n",
            ),
            18740,
            1140,
        ),
    ];

    for (input, part1_result, part2_result) in tests.iter() {
        let mut battle1: Battle = input.parse().unwrap();
        let battle2 = battle1.clone();
        battle1.run();
        assert_eq!(battle1.get_outcome(), *part1_result);
        assert_eq!(part2(&battle2), *part2_result);
    }
}
