use crate::util::Day;

const DAY_NR: u8 = 1;
const PROBLEM_TITLE: &str = "Historian Hysteria";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let mut distance = 0;
    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
    }

    distance.to_string()
}

fn solve_part2(input: &str) -> String {
    let (left, right) = parse_input(input);

    let mut similarity = 0;
    for loc in left {
        similarity += loc * right.iter().filter(|n| **n == loc).count() as u32;
    }

    similarity.to_string()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        left_list.push(parts[0].parse().expect("Error parsing integer"));
        right_list.push(parts[1].parse().expect("Error parsing integer"));
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#;
        assert_eq!(solve_part1(input), "11");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "1110981");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#;
        assert_eq!(solve_part2(input), "31");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "24869388");
    }
}
