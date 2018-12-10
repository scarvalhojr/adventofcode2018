use clap::{crate_description, App, Arg};
use day08::{part1, part2, NodeGraph};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
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
    let input = read_input(args.value_of("INPUT").unwrap());
    let graph = NodeGraph::build_graph(&input);
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

fn read_input(filename: &str) -> Vec<u8> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
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

    match contents
        .split(' ')
        .map(|number| {
            number
                .parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .collect()
    {
        Ok(numbers) => numbers,
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
