use clap::{crate_description, App, Arg};
use day07::{Steps, Workers};
use std::fs::File;
use std::io::Read;
use std::process::exit;

const NUM_WORKERS: usize = 5;
const BASE_STEP_TIME: u32 = 60;

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
    let mut steps = read_input(args.value_of("INPUT").unwrap());
    let mut workers = Workers::new(steps.clone(), NUM_WORKERS, BASE_STEP_TIME);
    println!("Part 1: {}", steps.get_serial_order());
    println!("Part 2: {}", workers.time_all_steps());
}

fn read_input(filename: &str) -> Steps {
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
        Ok(steps) => steps,
        Err(err) => {
            println!("Failed to parse input file '{}': {}", filename, err);
            exit(4);
        }
    }
}
