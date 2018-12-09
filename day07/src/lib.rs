use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::str::FromStr;

type Step = char;

fn step_time(step: Step) -> u32 {
    if step as u32 >= 'A' as u32 {
        1 + step as u32 - 'A' as u32
    } else {
        0
    }
}

#[derive(Clone)]
pub struct Steps {
    pending: HashMap<Step, HashSet<Step>>,
}

impl Steps {
    pub fn get_serial_order(&mut self) -> String {
        let mut order = Vec::new();
        while let Some(step) = self.start_step() {
            order.push(step);
            self.complete_step(step);
        }
        order.iter().collect()
    }

    fn start_step(&mut self) -> Option<Step> {
        let step = self
            .pending
            .iter()
            .filter(|(_, blockers)| blockers.is_empty())
            .map(|(step, _)| *step)
            .min()?;
        self.pending.remove(&step);
        Some(step)
    }

    fn complete_step(&mut self, step: Step) {
        for blockers in self.pending.values_mut() {
            blockers.remove(&step);
        }
    }
}

impl FromStr for Steps {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(
            r"^Step ([[:alpha:]]) must be .* step ([[:alpha:]]) can begin.$",
        )
        .unwrap();

        let mut pending = HashMap::new();
        for line in s.lines() {
            let captures = pattern.captures(line).ok_or("Invalid format")?;
            let step: Vec<Step> = captures
                .iter()
                .skip(1)
                .map(|val| val.unwrap().as_str().chars().next().unwrap())
                .collect();

            pending.entry(step[0]).or_insert_with(HashSet::new);
            pending
                .entry(step[1])
                .and_modify(|blockers| {
                    blockers.insert(step[0]);
                })
                .or_insert_with(|| [step[0]].iter().cloned().collect());
        }

        Ok(Self { pending })
    }
}

type Time = u32;

pub struct Workers {
    steps: Steps,
    worker_state: Vec<Option<(Step, Time)>>,
    curr_time: Time,
    time_events: BTreeSet<Time>,
    base_step_time: u32,
}

impl Workers {
    pub fn new(steps: Steps, num_workers: usize, base_step_time: u32) -> Self {
        Workers {
            steps,
            worker_state: vec![None; num_workers],
            time_events: [0].iter().cloned().collect(),
            curr_time: 0,
            base_step_time,
        }
    }

    pub fn time_all_steps(&mut self) -> Time {
        while let Some(time) = self.time_events.iter().next() {
            self.curr_time = *time;
            self.time_events.remove(&self.curr_time);
            self.complete_steps();
            self.start_steps();
        }
        self.curr_time
    }

    fn start_steps(&mut self) {
        let end_time = self.curr_time + self.base_step_time;

        for worker in self.worker_state.iter_mut().filter(|w| w.is_none()) {
            if let Some(step) = self.steps.start_step() {
                let step_end = end_time + step_time(step);
                self.time_events.insert(step_end);
                *worker = Some((step, step_end));
            } else {
                break;
            }
        }
    }

    fn complete_steps(&mut self) {
        let curr_time = self.curr_time;

        for worker in self.worker_state.iter_mut() {
            if let Some((step, end_time)) = worker {
                if *end_time == curr_time {
                    self.steps.complete_step(*step);
                    *worker = None;
                }
            }
        }
    }
}
