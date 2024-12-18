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
    let output = computer.run();
    output.to_string()
}

fn solve_part2(input: &str) -> String {
    let computer = Computer::new(input);
    let reg_a = part2_from_felix(computer);
    reg_a.to_string()
}

#[derive(Debug, FromPrimitive)]
enum OpCode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

#[derive(Debug)]
struct Operation {
    instruction: OpCode,
    operand: u8,
}

#[derive(Debug)]
struct Computer {
    program: Vec<Operation>,
    program_raw: Vec<u64>,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
}

impl Computer {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();

        // parse registers
        let mut reg_a: Option<u64> = None;
        let mut reg_b: Option<u64> = None;
        let mut reg_c: Option<u64> = None;
        let re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
        for (_, [reg, val]) in re.captures_iter(parts[0]).map(|c| c.extract()) {
            match reg {
                "A" => reg_a = Some(val.parse::<u64>().unwrap()),
                "B" => reg_b = Some(val.parse::<u64>().unwrap()),
                "C" => reg_c = Some(val.parse::<u64>().unwrap()),
                _ => panic!("Unexpected register"),
            }
        }

        // parse program
        let mut program = vec![];
        let mut program_raw: Vec<u64> = vec![];
        let program_str = parts[1].split_whitespace().collect::<Vec<&str>>()[1];
        let mut iter = program_str.split(',');
        loop {
            match iter.next() {
                None => break,
                Some(opcode_str) => {
                    let opcode = opcode_str.parse::<u8>().unwrap();
                    let instruction: OpCode = num::FromPrimitive::from_u8(opcode).unwrap();

                    let operand = iter.next().unwrap().parse::<u8>().unwrap();
                    program_raw.push(opcode as u64);
                    program_raw.push(operand as u64);
                    program.push(Operation {
                        instruction,
                        operand,
                    });
                }
            }
        }

        Computer {
            program,
            program_raw,
            reg_a: reg_a.unwrap(),
            reg_b: reg_b.unwrap(),
            reg_c: reg_c.unwrap(),
            ip: 0,
        }
    }

    fn operand_value(&self, operand: u8) -> u64 {
        if operand < 4 {
            return operand as u64;
        }
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
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
                OpCode::ADV => {
                    let nom = self.reg_a;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_a = nom / denom;
                }

                OpCode::BXL => {
                    self.reg_b = self.reg_b ^ op.operand as u64;
                }

                OpCode::BST => {
                    self.reg_b = self.operand_value(op.operand) % 8;
                }

                OpCode::JNZ => {
                    if self.reg_a != 0 {
                        assert!(op.operand % 2 == 0);
                        self.ip = op.operand as usize / 2;
                        jumped = true;
                    }
                }

                OpCode::BXC => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                    // igore operand
                }

                OpCode::OUT => {
                    let val = self.operand_value(op.operand) % 8;
                    output.push(val);
                }

                OpCode::BDV => {
                    let nom = self.reg_a;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_b = nom / denom;
                }

                OpCode::CDV => {
                    let nom = self.reg_a;
                    let denom = 2u64.pow(self.operand_value(op.operand) as u32);
                    self.reg_c = nom / denom;
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

fn compute_fast(mut a: u64) -> Vec<u64> {
    // bst 4(A)         => B = A mod 8
    // bxl 1(1)         => B = B xor 1
    // cdv 5(B)         => C = A / 2^B
    // bxc 6(ignored)   => B = B xor C
    // adv 3(3)         => A = A / 2^3
    // bxl 4(4)         => B = B xor 4
    // out 5(B)
    // jnz 0

    let mut out = vec![];
    let mut b;
    let mut c;
    while a != 0 {
        b = a % 8;
        b = b ^ 1;
        c = a >> b;
        b = b ^ c;
        a = a >> 3;
        b = b ^ 4;
        out.push(b % 8);
    }
    out
}

fn part2_from_felix(computer: Computer) -> u64 {
    let mut a = 0;
    let prog_len = computer.program_raw.len();
    for i in 0..prog_len {
        let expected_out = &computer.program_raw[prog_len - i - 1..];
        let mut offset = 0;
        loop {
            let next_a = (a << 3) + offset;
            let result = compute_fast(next_a);
            if result == expected_out {
                a = next_a;
                break;
            }
            offset += 1;
        }
    }
    a
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

    fn example2() -> &'static str {
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
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
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "202972175280682");
    }
}
