use std::iter::repeat_n;

use itertools::{enumerate, Itertools};

use crate::util::Day;

const DAY_NR: u8 = 7;
const PROBLEM_TITLE: &str = "Bridge Repair";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

type Num = u128;

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

    fn can_be_true(&self) -> bool {
        let nr_ops = self.numbers.len() as u32 - 1;

        println!("Nr of ops: {}", nr_ops);

        let x = (2 as Num).pow(nr_ops);

        for ops in 0..x {
            let mut result = self.numbers[0];
            println!("{:b}", ops);
            let mut mask = (2 as Num).pow(nr_ops - 1);
            for i in 0..nr_ops {
                print!("{:b} ", mask);
                let times = ops & mask != 0;
                if times {
                    print!("* ");
                    result *= self.numbers[(i + 1) as usize];
                } else {
                    print!("+ ");
                    result += self.numbers[(i + 1) as usize];
                }
                mask >>= 1;
            }
            println!("   Result: {}", result);
            if result == self.test_value {
                println!("True !");
                return true;
            }
        }

        false
    }

    fn can_be_true2(&self) -> bool {
        let nr_ops = self.numbers.len() as u32 - 1;

        println!("Nr of ops: {}", nr_ops);

        for ops in repeat_n([0, 1], nr_ops as usize).multi_cartesian_product() {
            print!("{:?}", ops);

            let mut result = self.numbers[0];
            for i in 0..ops.len() {
                match ops[i] {
                    0 => result += self.numbers[i + 1],
                    1 => result *= self.numbers[i + 1],
                    _ => unreachable!(),
                }
            }
            if result == self.test_value {
                return true;
            }
        }

        false
    }

    fn can_be_true3(&self) -> bool {
        let nr_ops = self.numbers.len() as u32 - 1;

        println!("Nr of ops: {}", nr_ops);

        for ops in repeat_n([0, 1, 2], nr_ops as usize).multi_cartesian_product() {
            print!("{:?}", ops);

            let mut result = self.numbers[0];
            for i in 0..ops.len() {
                match ops[i] {
                    0 => result += self.numbers[i + 1],
                    1 => result *= self.numbers[i + 1],
                    2 => {
                        println!(
                            "Res: {}, Next: {}, log10(Next): {}",
                            result,
                            self.numbers[i + 1],
                            self.numbers[i + 1].ilog10()
                        );
                        result = result * (10 as Num).pow(self.numbers[i + 1].ilog10() + 1)
                            + self.numbers[i + 1]
                    }
                    _ => unreachable!(),
                }
                // if result > self.test_value {
                //     break;
                // }
            }
            println!("  {}", result);
            if result == self.test_value {
                return true;
            }
        }

        false
    }
}

fn solve_part1(input: &str) -> String {
    let equations: Vec<Equation> = input.lines().map(|line| Equation::new(line)).collect();
    println!("{:?}", equations);

    equations
        .into_iter()
        .map(|e| match e.can_be_true2() {
            true => e.test_value,
            false => 0,
        })
        .sum::<Num>()
        .to_string()
}

// fn is_valid(&self, operations: &[fn(u64, u64) -> u64]) -> bool {
//         let missing = 3;
//         // This creates all possible sequences with repetition
//         repeat_n(operations, missing)
//             .multi_cartesian_product()
//             .any(|ops| {
//                 ops.iter()
//                     .zip(self.operands[1..].iter())
//                     .fold(self.operands[0], |acc, (op, operand)| op(acc, *operand))
//                     == self.result
//             })
//     }
// }

fn solve_part2(input: &str) -> String {
    let equations: Vec<Equation> = input.lines().map(|line| Equation::new(line)).collect();
    println!("{:?}", equations);

    equations
        .into_iter()
        .map(|e| match e.can_be_true3() {
            true => e.test_value,
            false => 0,
        })
        .sum::<Num>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve_part1(input), "3749");
    }

    #[test]
    fn test_part1_with_simple_example() {
        let input = "25: 2 3 19";
        solve_part1(input);
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "850435817339");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve_part2(input), "11387");
    }

    #[test]
    fn test_part2_with_simple_example() {
        let input = "7290: 6 8 6 15";
        solve_part2(input);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "104824810233437");
    }
}
