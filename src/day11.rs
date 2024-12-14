use core::num;
use std::collections::HashMap;

use crate::util::Day;

const DAY_NR: u8 = 11;
const PROBLEM_TITLE: &str = "Plutonian Pebbles";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    blink(input, 25).to_string()
}

fn solve_part2(input: &str) -> String {
    blink(input, 75).to_string()
}

fn blink(input: &str, blinks: u32) -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for stone in input.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        stones.insert(stone, 1u64);
    }

    for _ in 0..blinks {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (stone, number) in stones.into_iter() {
            if stone == 0 {
                add(&mut new_stones, 1, number);
            } else {
                let digits = digits(stone);
                if digits % 2 == 0 {
                    let pow = 10u64.pow(digits / 2);
                    add(&mut new_stones, stone / pow, number);
                    add(&mut new_stones, stone % pow, number);
                } else {
                    add(&mut new_stones, stone * 2024, number);
                }
            }
        }
        stones = new_stones;
    }

    let mut count = 0;
    for (_stone, number) in stones.into_iter() {
        count += number;
    }

    count
}

fn add(stones: &mut HashMap<u64, u64>, stone: u64, number: u64) {
    let total_number = match stones.get(&stone) {
        None => number,
        Some(x) => x + number,
    };
    stones.insert(stone, total_number);
}

fn digits(n: u64) -> u32 {
    let mut power = 10;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(10) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "125 17"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "55312");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "211306");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "65601038650482");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "250783680217283");
    }
}
