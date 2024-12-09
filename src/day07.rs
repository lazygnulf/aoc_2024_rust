use std::iter::repeat_n;

use itertools::Itertools;

use crate::util::Day;

const DAY_NR: u8 = 7;
const PROBLEM_TITLE: &str = "Bridge Repair";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

type Num = u64;

#[derive(Debug)]
struct Equation {
    test_value: Num,
    numbers: Vec<Num>,
}

impl Equation {
    fn new(s: &str) -> Self {
        let parts = s.split(": ").collect::<Vec<&str>>();

        let test_value: Num = parts[0].parse().expect("Error parsing integer.");
        let numbers: Vec<Num> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().expect("Error parsing integer."))
            .collect();

        Equation {
            test_value,
            numbers,
        }
    }

    fn can_be_true(&self, is_part1: bool) -> bool {
        let op_selection: Vec<u8> = match is_part1 {
            true => vec![0, 1],
            false => vec![0, 1, 2],
        };

        let nr_ops = self.numbers.len() as u32 - 1;

        for ops in repeat_n(op_selection, nr_ops as usize).multi_cartesian_product() {
            let mut result = self.numbers[0];
            for i in 0..ops.len() {
                match ops[i] {
                    0 => result += self.numbers[i + 1],
                    1 => result *= self.numbers[i + 1],
                    2 => {
                        result = result * (10 as Num).pow(self.numbers[i + 1].ilog10() + 1)
                            + self.numbers[i + 1]
                    }
                    _ => unreachable!(),
                }
                if result > self.test_value {
                    break;
                }
            }
            if result == self.test_value {
                return true;
            }
        }

        false
    }
}

fn solve_part1(input: &str) -> String {
    solve(input, true)
}

fn solve_part2(input: &str) -> String {
    solve(input, false)
}

fn solve(input: &str, is_part1: bool) -> String {
    let equations: Vec<Equation> = input.lines().map(|line| Equation::new(line)).collect();

    equations
        .into_iter()
        .map(|e| match e.can_be_true(is_part1) {
            true => e.test_value,
            false => 0,
        })
        .sum::<Num>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "3749");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "850435817339");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "11387");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "104824810233437");
    }
}
