use crate::util::Day;

const DAY_NR: u8 = XX;
const PROBLEM_TITLE: &str = "TBD";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    println!("{}", input);

    "17".to_string()
}

fn solve_part2(input: &str) -> String {
    println!("{}", input);

    "42".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "17");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
