use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

type GuardID = u32;
type Minute = usize;
type MinuteCounters = HashMap<GuardID, Vec<u32>>;

pub enum Event {
    ShiftBegin(GuardID),
    FallAsleep(Minute),
    WakeUp(Minute),
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event_regex =
            Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\]\s+(.*)$")
                .unwrap();

        let parts = event_regex
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let minute = parts
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        let event_str = parts.get(2).unwrap().as_str();
        let shift_regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

        if let Some(shift_info) = shift_regex.captures(event_str) {
            let guard_id = shift_info
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
            Ok(Event::ShiftBegin(guard_id))
        } else {
            match event_str {
                "falls asleep" => Ok(Event::FallAsleep(minute)),
                "wakes up" => Ok(Event::WakeUp(minute)),
                _ => Err(Error::new(ErrorKind::InvalidData, "Unknown event")),
            }
        }
    }
}

pub fn process_events(events: Vec<Event>) -> MinuteCounters {
    let mut sleep_counters: MinuteCounters = HashMap::new();
    let mut curr_guard = None;
    let mut sleep_start = None;

    for event in events {
        match event {
            Event::ShiftBegin(guard_id) => {
                assert!(sleep_start.is_none());
                curr_guard = Some(guard_id);
            }
            Event::FallAsleep(sleep_minute) => {
                assert!(curr_guard.is_some() && sleep_start.is_none());
                sleep_start = Some(sleep_minute);
            }
            Event::WakeUp(wake_minute) => {
                let guard_id = curr_guard.unwrap();
                let sleep_min = sleep_start.unwrap();
                for item in sleep_counters
                    .entry(guard_id)
                    .or_insert(vec![0; 60])
                    .iter_mut()
                    .take(wake_minute)
                    .skip(sleep_min)
                {
                    *item += 1;
                }
                sleep_start = None;
            }
        }
    }

    sleep_counters
}

pub fn part1(sleep_counters: &MinuteCounters) -> u32 {
    match sleep_counters
        .iter()
        .map(|(guard_id, counters)| (*guard_id, counters.iter().sum::<u32>()))
        .max_by_key(|(_, total_sleep)| *total_sleep)
    {
        None => 0,
        Some((guard_id, _)) => {
            let (sleepiest_minute, _) = sleep_counters
                .get(&guard_id)
                .unwrap()
                .iter()
                .enumerate()
                .max_by_key(|(_, sleeping_count)| *sleeping_count)
                .unwrap();
            guard_id * sleepiest_minute as u32
        }
    }
}

pub fn part2(sleep_counters: &MinuteCounters) -> u32 {
    match sleep_counters
        .iter()
        .map(|(guard_id, counters)| {
            let (sleepiest_minute, sleeping_count) = counters
                .iter()
                .enumerate()
                .max_by_key(|(_, sleeping_count)| *sleeping_count)
                .unwrap();
            (*guard_id, sleepiest_minute as u32, sleeping_count)
        })
        .max_by_key(|(_, _, sleeping_count)| *sleeping_count)
    {
        None => 0,
        Some((guard_id, sleepiest_minute, _)) => guard_id * sleepiest_minute,
    }
}
