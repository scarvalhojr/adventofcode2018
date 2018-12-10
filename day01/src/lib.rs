use std::collections::HashSet;

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> i32 {
    let mut set = HashSet::new();

    input
        .iter()
        .cycle()
        .scan(0, |acc, val| {
            *acc += val;
            Some(*acc)
        })
        .skip_while(|acc| set.insert(*acc))
        .next()
        .unwrap()
}
