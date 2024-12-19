use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
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
        8 => day08::get_day().solve(),
        9 => day09::get_day().solve(),
        10 => day10::get_day().solve(),
        11 => day11::get_day().solve(),
        13 => day13::get_day().solve(),
        14 => day14::get_day().solve(),
        15 => day15::get_day().solve(),
        16 => day16::get_day().solve(),
        17 => day17::get_day().solve(),
        18 => day18::get_day().solve(),
        19 => day19::get_day().solve(),

        _ => panic!("No implementation found."),
    }
}
