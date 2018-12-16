use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub struct Rule {
    pattern: Vec<bool>,
    result: bool,
}

pub struct Pots {
    states: VecDeque<bool>,
    start_idx: i64,
    live_rules: Vec<bool>,
}

impl Pots {
    pub fn add_rules(&mut self, rules: Vec<Rule>) {
        for rule in rules {
            let rule_idx: usize = [16, 8, 4, 2, 1]
                .iter()
                .zip(rule.pattern.iter())
                .filter(|(_, &state)| state)
                .map(|(base, _)| *base)
                .sum();
            self.live_rules[rule_idx] = rule.result;
        }
    }

    pub fn update_generations(&mut self, num_gen: usize) {
        let mut known_states: HashSet<Vec<bool>> = HashSet::new();
        let mut pattern: usize;
        let mut start_delta;

        known_states.insert(self.states.iter().cloned().collect());
        for gen in 1..=num_gen {
            pattern = [16, 8, 4, 2]
                .iter()
                .zip(self.states.iter().take(4))
                .filter(|(_, &state)| state)
                .map(|(base, _)| *base)
                .sum();
            for idx in 2..self.states.len() - 2 {
                if self.states[idx + 2] {
                    pattern += 1;
                }
                self.states[idx] = self.live_rules[pattern];
                pattern = (2 * pattern) % 32;
            }

            if self.states[2] {
                self.states.push_front(false);
                start_delta = -1;
            } else if !self.states[3] {
                self.states.pop_front();
                start_delta = 1;
            } else {
                start_delta = 0;
            }
            self.start_idx += start_delta;

            if self.states[self.states.len() - 3] {
                self.states.push_back(false);
            } else if !self.states[self.states.len() - 4] {
                self.states.pop_back();
            }

            if !known_states.insert(self.states.iter().cloned().collect()) {
                // Cycle detected: this only works if cycle is of length 1
                self.start_idx += start_delta * (num_gen - gen) as i64;
                return;
            }
        }
    }

    pub fn get_sum_live_pots(&self) -> i64 {
        self.states
            .iter()
            .enumerate()
            .filter(|(_, &state)| state)
            .map(|(idx, _)| idx as i64 + self.start_idx)
            .sum::<i64>()
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display: String = self
            .states
            .iter()
            .map(|state| match state {
                true => '#',
                false => '.',
            })
            .collect();
        write!(f, "{}", display)
    }
}

impl FromStr for Pots {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^initial state: ([#\.]+)").unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let states: VecDeque<_> = [false, false, false]
            .iter()
            .cloned()
            .chain(
                groups
                    .get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|ch| match ch {
                        '#' => true,
                        _ => false,
                    })
                    .chain([false, false, false].iter().cloned()),
            )
            .collect();

        Ok(Pots {
            states,
            start_idx: -3,
            live_rules: vec![false; 32],
        })
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"^([#\.]{5}) => ([#\.])").unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let pattern = groups
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .map(|ch| match ch {
                '#' => true,
                _ => false,
            })
            .collect();

        let result = groups
            .get(2)
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .map(|ch| match ch {
                '#' => true,
                _ => false,
            })
            .unwrap();

        Ok(Rule { pattern, result })
    }
}
