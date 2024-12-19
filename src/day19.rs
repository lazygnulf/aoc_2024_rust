use crate::util::Day;

const DAY_NR: u8 = 19;
const PROBLEM_TITLE: &str = "Linen Layout";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let towels: Vec<String> = parts[0].split(", ").map(|t| t.to_owned()).collect();
    let designs: Vec<String> = parts[1].lines().map(|d| d.to_owned()).collect();

    designs
        .iter()
        .filter(|d| possible(d, &towels))
        .count()
        .to_string()
}

fn solve_part2(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let towels: Vec<String> = parts[0].split(", ").map(|t| t.to_owned()).collect();
    let designs: Vec<String> = parts[1].lines().map(|d| d.to_owned()).collect();

    designs
        .iter()
        .map(|d| ways(d, &towels))
        .sum::<usize>()
        .to_string()
}

fn possible(s: &String, l: &Vec<String>) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for towel in l {
            let towel_len = towel.len();
            if i >= towel_len && &s[i - towel_len..i] == towel && dp[i - towel_len] {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}

fn ways(s: &String, l: &Vec<String>) -> usize {
    let n = s.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for towel in l {
            let towel_len = towel.len();
            if i >= towel_len && &s[i - towel_len..i] == towel {
                dp[i] += dp[i - towel_len];
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "6");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "293");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "16");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "623924810770264");
    }
}
