use std::cmp::Ordering;

use crate::util::Day;

const DAY_NR: u8 = 5;
const PROBLEM_TITLE: &str = "Print Queue";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

#[derive(Debug)]
struct OrderRule {
    before: u32,
    after: u32,
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}
impl Update {
    fn check_order_rules(&self, order_rules: &Vec<OrderRule>) -> bool {
        for rule in order_rules {
            if self.pages.contains(&rule.before) && self.pages.contains(&rule.after) {
                let pos_before = self
                    .pages
                    .iter()
                    .position(|page| *page == rule.before)
                    .unwrap();
                let pos_after = self
                    .pages
                    .iter()
                    .position(|page| *page == rule.after)
                    .unwrap();

                if pos_before > pos_after {
                    return false;
                }
            }
        }
        true
    }

    fn middle_page(&self) -> u32 {
        assert!(self.pages.len() % 2 == 1);
        self.pages[self.pages.len() / 2]
    }
}

fn solve_part1(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let order_rules: Vec<OrderRule> = parts[0]
        .lines()
        .map(|rule| {
            let pages = rule.split('|').collect::<Vec<&str>>();
            OrderRule {
                before: pages[0].parse::<u32>().unwrap(),
                after: pages[1].parse::<u32>().unwrap(),
            }
        })
        .collect();

    let updates: Vec<Update> = parts[1]
        .lines()
        .map(|line| {
            let pages = line
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            Update { pages }
        })
        .collect();

    let mut result: u32 = 0;
    for update in updates {
        if update.check_order_rules(&order_rules) {
            result += update.middle_page();
        }
    }

    result.to_string()
}

fn cmp(order_rules: &Vec<OrderRule>, a: &u32, b: &u32) -> Ordering {
    for rule in order_rules {
        if rule.before == *a && rule.after == *b {
            return Ordering::Less;
        } else if rule.before == *b && rule.after == *a {
            return Ordering::Greater;
        }
    }
    panic!("no rule found");
}

fn solve_part2(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let order_rules: Vec<OrderRule> = parts[0]
        .lines()
        .map(|rule| {
            let pages = rule.split('|').collect::<Vec<&str>>();
            OrderRule {
                before: pages[0].parse::<u32>().unwrap(),
                after: pages[1].parse::<u32>().unwrap(),
            }
        })
        .collect();

    let updates: Vec<Update> = parts[1]
        .lines()
        .map(|line| {
            let pages = line
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            Update { pages }
        })
        .collect();

    let mut result: u32 = 0;
    for update in updates {
        if !update.check_order_rules(&order_rules) {
            let mut fixed_update = update.pages.clone();
            fixed_update.sort_by(|a, b| cmp(&order_rules, a, b));
            result += fixed_update[fixed_update.len() / 2];
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(solve_part1(input), "143");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "6051");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(solve_part2(input), "123");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "5093");
    }
}
