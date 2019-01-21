pub fn reduced_polymer_len(polymer: &str) -> usize {
    reduced_len(polymer.chars())
}

fn reduced_len<I>(polymer: I) -> usize
where
    I: Iterator<Item = char>,
{
    let mut reduced = Vec::new();
    let mut last_unit: Option<char> = None;

    for unit in polymer {
        if let Some(last) = last_unit {
            if last.to_ascii_lowercase() == unit.to_ascii_lowercase()
                && last != unit
            {
                last_unit = reduced.pop();
            } else {
                reduced.push(last);
                last_unit = Some(unit);
            }
        } else {
            last_unit = Some(unit);
        }
    }

    if let Some(last) = last_unit {
        reduced.push(last);
    }

    reduced.len()
}

pub fn shortest_reduction(polymer: &str) -> usize {
    let mut unique_units: Vec<char> = polymer.to_lowercase().chars().collect();
    unique_units.sort_unstable();
    unique_units.dedup();

    unique_units
        .iter()
        .map(|&unit| {
            reduced_len(
                polymer.chars().filter(|u| u.to_ascii_lowercase() != unit),
            )
        })
        .min()
        .unwrap_or(0)
}
