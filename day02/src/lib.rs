use std::collections::HashMap;

pub fn part1(input: &[String]) -> u32 {
    let (total_dbl, total_tpl) = input
        .iter()
        .map(|s| has_double_and_triple(s))
        .fold((0, 0), |(dbl_count, tpl_count), (has_dbl, has_tpl)| {
            (dbl_count + has_dbl as u32, tpl_count + has_tpl as u32)
        });

    total_dbl * total_tpl
}

fn has_double_and_triple(word: &str) -> (bool, bool) {
    let mut counters = HashMap::new();
    for ch in word.chars() {
        *counters.entry(ch).or_insert(0) += 1;
    }
    (
        counters.values().any(|&c| c == 2),
        counters.values().any(|&c| c == 3),
    )
}

pub fn part2(input: &[String]) -> String {
    for (idx1, str1) in input.iter().enumerate() {
        let str1_len = str1.chars().count();
        for str2 in input[idx1..].iter() {
            let equal = equal_chars(str1, str2);
            if equal.len() == str1_len - 1 {
                return equal.into_iter().collect();
            }
        }
    }
    "".to_string()
}

fn equal_chars(str1: &str, str2: &str) -> Vec<char> {
    str1.chars()
        .zip(str2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect()
}
