use crate::util::Day;

const DAY_NR: u8 = 2;
const PROBLEM_TITLE: &str = "Red-Nosed Reports";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| is_safe(line))
        .count()
        .to_string()
}

fn solve_part2(_input: &str) -> String {
    "42".to_string()
}

fn is_safe(report: &str) -> bool {
    let levels: Vec<u16> = report
        .split_whitespace()
        .map(|level| level.parse().expect("Error parsing integer"))
        .collect();

    let mut increasing: Option<bool> = None;

    for pair in levels.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if diff < 1 || diff > 3 {
            return false;
        }
        match increasing {
            Some(v) => {
                if (pair[0] < pair[1]) != v {
                    return false;
                }
            }
            None => increasing = Some(pair[0] < pair[1]),
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = r#"7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"#;

        assert_eq!(solve_part1(input), "2");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "463");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = r#"7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"#;

        assert_eq!(solve_part2(input), "4");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
