use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

const OPEN: u8 = 0;
const TREES: u8 = 1;
const LUMBER: u8 = 2;

#[derive(Eq, Clone)]
pub struct Area {
    state: HashMap<(i32, i32), u8>,
    num_rows: i32,
    num_cols: i32,
}

impl Hash for Area {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num_rows.hash(state);
        self.num_cols.hash(state);
        self.get_states().hash(state);
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Area) -> bool {
        self.get_states() == other.get_states()
            && self.num_rows == other.num_rows
            && self.num_cols == other.num_cols
    }
}

impl Area {
    fn get_states(&self) -> Vec<u8> {
        (0..self.num_rows)
            .flat_map(|row| {
                (0..self.num_cols).map(move |col| {
                    *self.state.get(&(row, col)).unwrap_or(&OPEN)
                })
            })
            .collect::<Vec<_>>()
    }

    fn next_area(&self) -> Area {
        let mut new_state = HashMap::new();

        let count = |row, col| {
            (row - 1..=row + 1)
                .filter_map(|r| self.state.get(&(r, col)))
                .fold((0, 0), |(trees, lumber), state| match *state {
                    TREES => (trees + 1, lumber),
                    _ => (trees, lumber + 1),
                })
        };

        for row in 0..self.num_rows {
            let (mut trees, mut lumber) = count(row, 0);
            for col in 0..self.num_cols {
                let add = count(row, col + 1);
                let sub = count(row, col - 2);
                trees += add.0 - sub.0;
                lumber += add.1 - sub.1;
                match *self.state.get(&(row, col)).unwrap_or(&OPEN) {
                    OPEN => {
                        if trees >= 3 {
                            new_state.insert((row, col), TREES);
                        }
                    }
                    TREES => {
                        if lumber >= 3 {
                            new_state.insert((row, col), LUMBER);
                        } else {
                            new_state.insert((row, col), TREES);
                        }
                    }
                    LUMBER => {
                        if trees > 0 && lumber > 1 {
                            new_state.insert((row, col), LUMBER);
                        }
                    }
                    s => panic!("Unexpected state: {}", s),
                }
            }
        }

        Area {
            state: new_state,
            num_rows: self.num_rows,
            num_cols: self.num_cols,
        }
    }

    fn total_resource(&self) -> u64 {
        let trees = self.state.values().filter(|&s| *s == TREES).count();
        let lumber = self.state.values().filter(|&s| *s == LUMBER).count();
        (trees * lumber) as u64
    }
}

pub fn part1_and_2(area: &Area, end_time: usize) -> u64 {
    let mut areas: HashMap<Area, usize> = HashMap::new();
    let mut prev_area = area.clone();
    let mut next_area;
    let mut time = 0;
    let start_cycle = loop {
        next_area = prev_area.next_area();
        match areas.entry(prev_area) {
            Entry::Vacant(entry) => entry.insert(time),
            Entry::Occupied(entry) => break *entry.get(),
        };
        time += 1;
        if time == end_time {
            return next_area.total_resource();
        }
        prev_area = next_area;
    };
    let equiv_time = start_cycle + (end_time - time) % (time - start_cycle);
    let end_area = areas.iter().find(|(_, &min)| min == equiv_time).unwrap().0;
    end_area.total_resource()
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = (0..self.num_rows)
            .map(|row| {
                (0..self.num_cols)
                    .map(|col| {
                        self.state.get(&(row, col)).map_or('.', |state| {
                            match *state {
                                TREES => '|',
                                _ => '#',
                            }
                        })
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", display)
    }
}

impl FromStr for Area {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^[\.#\|\n]+$").unwrap();
        if !pattern.is_match(s) {
            return Err("Invalid character in input".to_string());
        }

        let state = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(col_num, ch)| match ch {
                        '|' => Some(((row_num as i32, col_num as i32), TREES)),
                        '#' => Some(((row_num as i32, col_num as i32), LUMBER)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let num_rows = 1 + state.keys().map(|(r, _)| *r).max().unwrap();
        let num_cols = 1 + state.keys().map(|(_, c)| *c).max().unwrap();

        Ok(Area {
            state,
            num_rows,
            num_cols,
        })
    }
}
