use std::collections::HashSet;

use pathfinding::prelude::astar;

use crate::util::Day;

const DAY_NR: u8 = 18;
const PROBLEM_TITLE: &str = "RAM Run";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    solve_part1_helper(input, 71, 71, 1024)
}

fn solve_part1_helper(input: &str, width: usize, height: usize, initial_bytes: usize) -> String {
    let mem = MemorySpace::new(width, height, initial_bytes, input);
    mem.minimum_steps_to_exit().unwrap().to_string()
}

fn solve_part2(input: &str) -> String {
    solve_part2_helper(input, 71, 71, 1024)
}

fn solve_part2_helper(input: &str, width: usize, height: usize, initial_bytes: usize) -> String {
    let mut mem = MemorySpace::new(width, height, initial_bytes, input);
    let Pos(x, y) = mem.find_pos_without_exit();
    format!("{},{}", x, y)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

#[derive(Debug)]
struct MemorySpace {
    width: usize,
    height: usize,
    initial_bytes: usize,
    corrupted: Vec<Pos>,
    obstacles: HashSet<Pos>,
}

impl MemorySpace {
    fn new(width: usize, height: usize, initial_bytes: usize, input: &str) -> Self {
        let mut corrupted = vec![];

        for line in input.lines() {
            let parts = line.split(',').collect::<Vec<&str>>();
            corrupted.push(Pos(
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            ));
        }

        let mut obstacles = HashSet::new();
        for i in 0..initial_bytes {
            obstacles.insert(corrupted[i].clone());
        }

        MemorySpace {
            width,
            height,
            initial_bytes,
            corrupted,
            obstacles,
        }
    }

    fn minimum_steps_to_exit(&self) -> Option<usize> {
        let start = Pos(0, 0);
        let goal = Pos(self.width as i32 - 1, self.height as i32 - 1);

        let res = astar(
            &start,
            |p| self.successors(p),
            |p| p.distance(&goal),
            |p| *p == goal,
        );

        match res {
            Some(path) => Some(path.0.len() - 1),
            None => None,
        }
    }

    fn find_pos_without_exit(&mut self) -> Pos {
        let mut next_pos;

        loop {
            next_pos = self.corrupted[self.initial_bytes].clone();
            self.obstacles.insert(next_pos.clone());

            match self.minimum_steps_to_exit() {
                Some(_input) => self.initial_bytes += 1,
                None => break,
            };
        }

        next_pos
    }

    fn successors(&self, p: &Pos) -> Vec<(Pos, u32)> {
        let mut succ = vec![];

        let &Pos(x, y) = p;
        let mut candidate;

        candidate = Pos(x - 1, y);
        if x > 0 && !self.obstacles.contains(&candidate) {
            succ.push((candidate, 1));
        }

        candidate = Pos(x + 1, y);
        if x + 1 < self.width as i32 && !self.obstacles.contains(&candidate) {
            succ.push((candidate, 1));
        }
        candidate = Pos(x, y - 1);
        if y > 0 && !self.obstacles.contains(&candidate) {
            succ.push((candidate, 1));
        }
        candidate = Pos(x, y + 1);
        if y + 1 < self.height as i32 && !self.obstacles.contains(&candidate) {
            succ.push((candidate, 1));
        }
        succ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1_helper(example(), 7, 7, 12), "22");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "454");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "6,1");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
