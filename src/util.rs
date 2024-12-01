use core::fmt;
use std::fs;

pub type Solver = fn(&str) -> String;

pub struct Day {
    number: u8,
    title: String,
    solver_part1: Solver,
    solver_part2: Solver,
}

impl Day {
    pub fn new(number: u8, title: &str, solver1: Solver, solver2: Solver) -> Self {
        Self {
            number,
            title: String::from(title),
            solver_part1: solver1,
            solver_part2: solver2,
        }
    }

    pub fn solve(&self) {
        println!("{}", self);
        let input = self.read_input();
        println!("Part 1: {}", (self.solver_part1)(&input));
        println!("Part 2: {}", (self.solver_part2)(&input));
    }

    pub fn read_input(&self) -> String {
        let path = fs::canonicalize(format!("input/input_{:0>2}.txt", self.number))
            .expect("Problem with input file path:");
        fs::read_to_string(path).expect("Failed to read input file:")
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--- Day {}: {} ---", self.number, self.title)
    }
}
