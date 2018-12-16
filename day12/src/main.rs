use clap::{crate_description, App, Arg};
use day12::{Pots, Rule};
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
        )
        .get_matches();

    println!(crate_description!());
    let mut pots = read_input(args.value_of("INPUT").unwrap());
    pots.update_generations(20);
    println!("Part 1: {}", pots.get_sum_live_pots());
    pots.update_generations(50_000_000_000 - 20);
    println!("Part 2: {}", pots.get_sum_live_pots());
}

fn read_input(filename: &str) -> Pots {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    match reader.read_line(&mut first_line) {
        Ok(_) => (),
        Err(err) => {
            println!(
                "Failed to read input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    };

    let mut pots: Pots = match first_line.parse() {
        Ok(pots) => pots,
        Err(err) => {
            println!(
                "Failed to parse input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    };

    let rules: Vec<Rule> = match reader
        .lines()
        .skip(1)
        .map(|line| {
            line.and_then(|value| {
                value
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            })
        })
        .collect()
    {
        Ok(rules) => rules,
        Err(err) => {
            println!(
                "Failed to parse input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    };

    pots.add_rules(rules);
    pots
}
