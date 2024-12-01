use std::env;

mod day01;
mod util;

fn main() {
    let mut day: u8 = 1;

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        day = args[1].parse::<u8>().expect("Invalid day format.");
    } else {
        println!("Usage: {} <day>", args[0])
    }

    println!("Advent of Code 2016");

    match day {
        1 => day01::get_day().solve(),
        _ => panic!("No implementation found."),
    }
}
