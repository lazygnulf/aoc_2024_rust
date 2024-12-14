use regex::Regex;

use crate::util::Day;

const DAY_NR: u8 = 13;
const PROBLEM_TITLE: &str = "Claw Contraption";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let machines = parse_input(input, true);

    let mut token: u32 = 0;

    for m in machines {
        let (a, b) = m.solve();
        if a.trunc() == a && b.trunc() == b {
            token += 3 * a as u32 + b as u32;
        }
    }

    token.to_string()
}

fn solve_part2(input: &str) -> String {
    let machines = parse_input(input, false);

    let mut token: i128 = 0;

    for m in machines {
        let (a, b) = m.solve();
        if a.trunc() == a && b.trunc() == b {
            token += 3 * a as i128 + b as i128;
        }
    }

    token.to_string()
}

fn parse_input(input: &str, is_part1: bool) -> Vec<ClawMachine> {
    let mut machines = vec![];

    // Button A: X+26, Y+66
    let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    // Prize: X=18641, Y=10279"
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    for machine_str in input.split("\n\n") {
        let mut a1: i128 = 0;
        let mut a2: i128 = 0;
        let mut b1: i128 = 0;
        let mut b2: i128 = 0;
        let mut c1: i128 = 0;
        let mut c2: i128 = 0;

        let mut l = machine_str.lines();

        for (_, [x, y]) in button_re
            .captures_iter(l.next().unwrap())
            .map(|c| c.extract())
        {
            a1 = x.parse::<i128>().unwrap();
            a2 = y.parse::<i128>().unwrap();
        }

        for (_, [x, y]) in button_re
            .captures_iter(l.next().unwrap())
            .map(|c| c.extract())
        {
            b1 = x.parse::<i128>().unwrap();
            b2 = y.parse::<i128>().unwrap();
        }

        for (_, [x, y]) in prize_re
            .captures_iter(l.next().unwrap())
            .map(|c| c.extract())
        {
            c1 = x.parse::<i128>().unwrap();
            c2 = y.parse::<i128>().unwrap();
            if !is_part1 {
                c1 += 10000000000000;
                c2 += 10000000000000;
            }
        }

        machines.push(ClawMachine {
            a1,
            a2,
            b1,
            b2,
            c1,
            c2,
        });
    }

    machines
}

#[derive(Debug)]
struct ClawMachine {
    a1: i128,
    a2: i128,
    b1: i128,
    b2: i128,
    c1: i128,
    c2: i128,
}

impl ClawMachine {
    fn det(&self) -> i128 {
        self.a1 * self.b2 - self.a2 * self.b1
    }

    fn solve(&self) -> (f64, f64) {
        let d = self.det() as f64;
        assert_ne!(d, 0.0);
        let dx = self.c1 * self.b2 - self.c2 * self.b1;
        let dy = self.a1 * self.c2 - self.a2 * self.c1;
        (dx as f64 / d, dy as f64 / d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "480");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "28138");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "875318608908");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "108394825772874");
    }
}
