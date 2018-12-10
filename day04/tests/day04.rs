use day04::{part1, part2, process_events, Event};

fn parse_input(input: &str) -> Vec<Event> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}

#[test]
fn example() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift
                 [1518-11-01 00:05] falls asleep
                 [1518-11-01 00:25] wakes up
                 [1518-11-01 00:30] falls asleep
                 [1518-11-01 00:55] wakes up
                 [1518-11-01 23:58] Guard #99 begins shift
                 [1518-11-02 00:40] falls asleep
                 [1518-11-02 00:50] wakes up
                 [1518-11-03 00:05] Guard #10 begins shift
                 [1518-11-03 00:24] falls asleep
                 [1518-11-03 00:29] wakes up
                 [1518-11-04 00:02] Guard #99 begins shift
                 [1518-11-04 00:36] falls asleep
                 [1518-11-04 00:46] wakes up
                 [1518-11-05 00:03] Guard #99 begins shift
                 [1518-11-05 00:45] falls asleep
                 [1518-11-05 00:55] wakes up";

    let sleep_counters = process_events(parse_input(&input));
    assert_eq!(part1(&sleep_counters), 240);
    assert_eq!(part2(&sleep_counters), 4455);
}
