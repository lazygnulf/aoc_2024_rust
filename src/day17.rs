use std::process::Output;

use num_derive::FromPrimitive;
use regex::Regex;

use crate::util::Day;

const DAY_NR: u8 = 17;
const PROBLEM_TITLE: &str = "Chronospatial Computer";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut computer = Computer::new(input);
    //    println!("{:?}", computer);

    let output = computer.run();

    output.to_string()
}

fn solve_part2(_input: &str) -> String {
    "42".to_string()
}

#[derive(Debug, FromPrimitive)]
enum OpCode {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

#[derive(Debug)]
struct Operation {
    instruction: OpCode,
    operand: u8,
}

#[derive(Debug)]
struct Computer {
    program: Vec<Operation>,
    reg_A: u64,
    reg_B: u64,
    reg_C: u64,
    ip: usize,
}

impl Computer {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();

        // parse registers
        let mut reg_A: Option<u64> = None;
        let mut reg_B: Option<u64> = None;
        let mut reg_C: Option<u64> = None;
        let re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
        for (_, [reg, val]) in re.captures_iter(parts[0]).map(|c| c.extract()) {
            match reg {
                "A" => reg_A = Some(val.parse::<u64>().unwrap()),
                "B" => reg_B = Some(val.parse::<u64>().unwrap()),
                "C" => reg_C = Some(val.parse::<u64>().unwrap()),
                _ => panic!("Unexpected register"),
            }
        }

        // parse program
        let mut program = vec![];
        let program_str = parts[1].split_whitespace().collect::<Vec<&str>>()[1];
        let mut iter = program_str.split(',');
        loop {
            match iter.next() {
                None => break,
                Some(code) => {
                    let instruction: OpCode =
                        num::FromPrimitive::from_u8(code.parse::<u8>().unwrap()).unwrap();
                    let operand = iter.next().unwrap().parse::<u8>().unwrap();
                    program.push(Operation {
                        instruction,
                        operand,
                    });
                }
            }
        }

        Computer {
            program,
            reg_A: reg_A.unwrap(),
            reg_B: reg_B.unwrap(),
            reg_C: reg_C.unwrap(),
            ip: 0,
        }
    }

    fn operand_value(&self, operand: u8) -> u64 {
        if operand < 4 {
            return operand as u64;
        }
        match operand {
            4 => self.reg_A,
            5 => self.reg_B,
            6 => self.reg_C,
            _ => panic!("unexpected operand"),
        }
    }

    fn run(&mut self) -> String {
        let mut output = vec![];
        self.ip = 0;

        while self.ip < self.program.len() {
            let mut jumped = false;

            let op = &self.program[self.ip];
            match op.instruction {
                OpCode::adv => {
                    let nom = self.reg_A;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_A = nom / denom;
                }

                OpCode::bxl => {
                    self.reg_B = self.reg_B ^ op.operand as u64;
                }

                OpCode::bst => {
                    self.reg_B = self.operand_value(op.operand) % 8;
                }

                OpCode::jnz => {
                    if self.reg_A != 0 {
                        assert!(op.operand % 2 == 0);
                        self.ip = op.operand as usize / 2;
                        jumped = true;
                    }
                }

                OpCode::bxc => {
                    self.reg_B = self.reg_B ^ self.reg_C;
                    // igore operand
                }

                OpCode::out => {
                    let val = self.operand_value(op.operand) % 8;
                    output.push(val);
                }

                OpCode::bdv => {
                    let nom = self.reg_A;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_B = nom / denom;
                }

                OpCode::cdv => {
                    let nom = self.reg_A;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_C = nom / denom;
                }

                _ => {
                    panic!("Unexpected instruction {:?} at {}", op, self.ip);
                }
            }

            if !jumped {
                self.ip += 1;
            }
        }

        output
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "7,3,0,5,7,1,4,0,5");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "42");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
