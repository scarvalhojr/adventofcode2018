use clap::{crate_description, App, Arg};
use day23::{part1, part2, Nanobot};
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
    let bots = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1(&bots));
    println!("Part 2: {}", part2(&bots));
}

fn read_input(filename: &str) -> Vec<Nanobot> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    match BufReader::new(file)
        .lines()
        .map(|line| {
            line.and_then(|value| {
                value
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            })
        })
        .collect()
    {
        Ok(points) => points,
        Err(err) => {
            println!(
                "Failed to parse input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    }
}
