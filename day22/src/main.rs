use clap::{crate_description, value_t_or_exit, App, Arg};
use day22::Cave;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("DEPTH")
                .help("Sets the depth of the cave")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("TARGET_X")
                .help("Sets the target X coordinate")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("TARGET_Y")
                .help("Sets the target Y coordinate")
                .required(true)
                .index(3),
        )
        .get_matches();

    println!(crate_description!());
    let depth = value_t_or_exit!(args.value_of("DEPTH"), u32);
    let target_x = value_t_or_exit!(args.value_of("TARGET_X"), u32);
    let target_y = value_t_or_exit!(args.value_of("TARGET_Y"), u32);
    let mut cave = Cave::new(depth, target_x, target_y);
    println!("Part 1: {}", cave.total_risk());
    println!("Part 2: {}", cave.min_distance());
}
