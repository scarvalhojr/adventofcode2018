use day20::{part1, Rooms};

#[test]
fn examples_part1() {
    let examples = [
        ("^WNE$", 3),
        ("^ENWWW(NEEE|SSE(EE|N))$", 10),
        ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
        ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
        (
            "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
            31,
        ),
    ];

    for example in examples.iter() {
        let rooms = Rooms::build(&example.0).unwrap();
        assert_eq!(part1(&rooms), example.1);
    }
}
