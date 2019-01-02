use clap::{crate_description, value_t_or_exit, App, Arg};
use day14::Scoreboard;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input number to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());

    let mut scoreboard: Scoreboard = Default::default();

    let num_recipes = value_t_or_exit!(args.value_of("INPUT"), usize);
    println!("Part 1: {}", scoreboard.get_score_after(num_recipes));

    let digits = args.value_of("INPUT").unwrap();
    println!("Part 1: {}", scoreboard.find_pattern(digits));
}
