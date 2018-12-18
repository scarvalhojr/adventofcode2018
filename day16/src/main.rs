use clap::{crate_description, App, Arg};
use day16::{part1, part2, Code, Sample};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("SAMPLES")
                .help("Sets the samples input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("CODE")
                .help("Sets the code input file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    println!(crate_description!());
    let samples = read_samples(args.value_of("SAMPLES").unwrap());
    println!("Part 1: {}", part1(&samples));
    let code = read_code(args.value_of("CODE").unwrap());
    println!("Part 2: {}", part2(&samples, &code));
}

fn read_samples(filename: &str) -> Vec<Sample> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    let lines: Vec<String> = match BufReader::new(file).lines().collect() {
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

    match lines
        .chunks(4)
        .map(|chunk| {
            chunk
                .concat()
                .parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .collect()
    {
        Ok(samples) => samples,
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

fn read_code(filename: &str) -> Vec<Code> {
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
        Ok(source) => source,
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
