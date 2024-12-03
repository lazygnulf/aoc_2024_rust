use crate::util::Day;
use regex::Regex;

const DAY_NR: u8 = 3;
const PROBLEM_TITLE: &str = "Mull It Over";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut result: u32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
        result += l.parse::<u32>().unwrap() * r.parse::<u32>().unwrap();
    }

    result.to_string()
}

fn solve_part2(input: &str) -> String {
    let mut result: u32 = 0;
    let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(don't\(\))|(do\(\))").unwrap();

    let mut active = true;
    for c in re.captures_iter(input) {
        dbg!(&c);
        match &c[0][..3] {
            "mul" => {
                if active {
                    result += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap();
                }
            }
            "do(" => active = true,
            "don" => active = false,
            _ => panic!("unexpected token"),
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve_part1(input), "161");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "175700056");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(input), "48");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "71668682");
    }
}
