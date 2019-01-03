use clap::{crate_description, App, Arg};
use day15::{part2, Battle};
use std::fs::File;
use std::io::Read;
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
    let mut battle1 = read_input(args.value_of("INPUT").unwrap());
    let battle2 = battle1.clone();
    battle1.run();
    println!("Part 1: {}", battle1.get_outcome());
    println!("Part 2: {}", part2(&battle2));
}

fn read_input(filename: &str) -> Battle {
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

    match contents.parse() {
        Ok(area) => area,
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
