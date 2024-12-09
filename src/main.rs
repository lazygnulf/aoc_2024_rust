use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
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
        2 => day02::get_day().solve(),
        3 => day03::get_day().solve(),
        4 => day04::get_day().solve(),
        5 => day05::get_day().solve(),
        6 => day06::get_day().solve(),
        7 => day07::get_day().solve(),
        _ => panic!("No implementation found."),
    }
}
