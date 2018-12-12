use clap::{crate_description, value_t_or_exit, App, Arg};
use day11::FuelGrid;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("SERIAL_NUMBER")
                .help("Fuel Grid serial number")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());
    let serial_num = value_t_or_exit!(args.value_of("SERIAL_NUMBER"), usize);
    let dimension = 300;
    let grid = FuelGrid::new(dimension, serial_num);

    let (pos_x, pos_y, _) = grid.max_square_size(3);
    println!("Part 1: {},{}", pos_x, pos_y);

    let (pos_x, pos_y, size, _) = grid.max_square();
    println!("Part 2: {},{},{}", pos_x, pos_y, size);
}
