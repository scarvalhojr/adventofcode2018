extern crate clap;
extern crate day04;

use clap::{crate_description, App, Arg};
use day04::{part1, part2, process_events, Event};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        ).get_matches();

    println!(crate_description!());
    let events = read_input(args.value_of("INPUT").unwrap());
    let sleep_counters = process_events(events);
    println!("Part 1: {}", part1(&sleep_counters));
    println!("Part 2: {}", part2(&sleep_counters));
}

fn read_input(filename: &str) -> Vec<Event> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    let mut lines: Vec<_> = match BufReader::new(file).lines().collect() {
        Ok(lines) => lines,
        Err(err) => {
            println!(
                "Failed to read input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    };

    lines.sort();

    match lines
        .iter()
        .map(|line| {
            line.parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        }).collect()
    {
        Ok(input) => input,
        Err(err) => {
            println!(
                "Failed to parse input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(4);
        }
    }
}
