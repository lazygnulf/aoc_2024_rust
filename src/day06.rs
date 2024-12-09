use core::str;
use std::collections::HashSet;

use crate::util::Day;

const DAY_NR: u8 = 6;
const PROBLEM_TITLE: &str = "Guard Gallivant";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

const GUARD: u8 = b'^';
const OBSTRUCTION: u8 = b'#';
const MARK: u8 = b'X';
const EMPTY: u8 = b'.';

fn solve_part1(input: &str) -> String {
    let mut map = Map::new(input);
    let mut guard = map.get_guard().expect("No guard found in map.");

    let mut visited: u32 = 1;

    while guard.is_in_map() {
        if guard.try_move() {
            if guard.visit() {
                visited += 1;
            }
        } else {
            guard.turn_right();
        }
    }

    visited.to_string()
}

fn solve_part2(input: &str) -> String {
    let mut result = 0;

    let map = Map::new(input);

    for row in 0..map.height() {
        for col in 0..map.width() {
            let mut test_map = Map::new(input);
            if test_map.place_obstruction(row, col) && test_map.check_cycle() {
                result += 1;
            }
        }
    }

    result.to_string()
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn new(s: &str) -> Map {
        Map {
            map: s
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>(),
        }
    }

    fn get_guard(&mut self) -> Option<Guard> {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.map[row][col] == GUARD {
                    self.mark(row as i32, col as i32);
                    return Some(Guard::new(self, row as i32, col as i32));
                }
            }
        }
        None
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn is_obstruction(&self, row: i32, col: i32) -> bool {
        self.map[row as usize][col as usize] == OBSTRUCTION
    }

    fn place_obstruction(&mut self, row: usize, col: usize) -> bool {
        if self.map[row][col] == EMPTY {
            self.map[row][col] = OBSTRUCTION;
            return true;
        }
        false
    }

    fn mark(&mut self, row: i32, col: i32) -> bool {
        if self.map[row as usize][col as usize] != MARK {
            self.map[row as usize][col as usize] = MARK;
            return true;
        }
        false
    }

    fn check_cycle(&mut self) -> bool {
        let mut guard = self.get_guard().expect("No guard found.");
        let mut trace = HashSet::new();

        while !trace.contains(&guard.pos_and_dir()) {
            let guard_pos_and_dir = guard.pos_and_dir();
            trace.insert(guard_pos_and_dir);
            while !guard.try_move() {
                if !guard.is_in_map() {
                    return false;
                } else {
                    guard.turn_right();
                }
            }
            if !guard.is_in_map() {
                return false;
            }
        }

        true
    }

    // fn print(&self) {
    //     for row in &self.map {
    //         println!("{}", String::from_utf8(row.clone()).unwrap());
    //     }
    // }
}

const DIRECTIONS: [[i32; 2]; 4] = [
    [-1, 0], // up
    [0, 1],  // right
    [1, 0],  // down
    [0, -1], // left
];

struct Guard<'a> {
    map: &'a mut Map,
    row: i32,
    col: i32,
    dir: usize, // index in DIRECTIONS array
}

impl Guard<'_> {
    fn new(map: &mut Map, row: i32, col: i32) -> Guard {
        Guard {
            map,
            row,
            col,
            dir: 0,
        }
    }

    fn try_move(&mut self) -> bool {
        let old_row = self.row;
        let old_col = self.col;

        self.row += DIRECTIONS[self.dir][0];
        self.col += DIRECTIONS[self.dir][1];

        if !self.is_in_map() {
            return false;
        }

        if self.map.is_obstruction(self.row, self.col) {
            self.row = old_row;
            self.col = old_col;
            return false;
        }

        true
    }

    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }

    fn is_in_map(&self) -> bool {
        self.row >= 0
            && self.row < self.map.height() as i32
            && self.col >= 0
            && self.col < self.map.width() as i32
    }

    fn visit(&mut self) -> bool {
        self.map.mark(self.row, self.col)
    }

    fn pos_and_dir(&self) -> ((i32, i32), usize) {
        ((self.row, self.col), self.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve_part1(input), "41");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "4967");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve_part2(input), "6");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "1789");
    }
}
