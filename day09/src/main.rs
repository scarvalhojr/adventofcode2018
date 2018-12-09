extern crate clap;
extern crate day09;

use clap::{crate_description, value_t_or_exit, App, Arg};
use day09::{part1, part2, MarbleGame};

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("NUM_PLAYERS")
                .help("Number of players")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("LAST_MARBLE")
                .help("Last marble to be played")
                .required(true)
                .index(2),
        ).get_matches();

    println!(crate_description!());
    let num_players = value_t_or_exit!(args.value_of("NUM_PLAYERS"), usize);
    let last_marble = value_t_or_exit!(args.value_of("LAST_MARBLE"), u32);
    let mut game = MarbleGame::new(num_players);
    println!("Part 1: {}", part1(&mut game, last_marble));
    println!("Part 2: {}", part2(&mut game, last_marble));
}
