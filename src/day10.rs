use std::collections::HashMap;

use crate::util::Day;

const DAY_NR: u8 = 10;
const PROBLEM_TITLE: &str = "Hoof It";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut topo = TopographicMap::new(input);
    topo.find_all_trails();

    println!("{:?}", topo);

    "36".to_string()
}

fn solve_part2(_input: &str) -> String {
    "42".to_string()
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

#[derive(Debug, Clone)]
struct Trail {
    positions: Vec<(Pos, u8)>,
}

impl Trail {
    fn new(head: Pos, height: u8) -> Trail {
        assert_eq!(height, 0);
        let positions = vec![(head, height)];
        Trail { positions }
    }
}

#[derive(Debug)]
struct TopographicMap {
    heights: HashMap<Pos, u8>,
    rows: usize,
    cols: usize,
    trail_heads: HashMap<Pos, Vec<Trail>>,
}

impl TopographicMap {
    fn new(input: &str) -> Self {
        let mut heights = HashMap::<Pos, u8>::new();

        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                heights.insert(Pos::new(row, col), ch.to_digit(10).unwrap() as u8);
                cols = col;
            }
            rows = row;
        }

        // TopographicMap {
        //     heights: input
        //         .lines()
        //         .map(|line| {
        //             line.chars()
        //                 .map(|ch| ch.to_digit(10).unwrap() as u8)
        //                 .collect::<Vec<u8>>()
        //         })
        //         .collect(),
        //     trails: vec![],
        // }

        TopographicMap {
            heights,
            rows,
            cols,
            trail_heads: HashMap::new(),
        }
    }

    fn find_all_trails(&mut self) {
        self.trail_heads = HashMap::new();

        // start by finding all zero height positions and create (empty) trail heads
        for (pos, height) in &self.heights {
            if *height == 0 {
                self.trail_heads
                    .insert(*pos, vec![Trail::new(*pos, *height)]);
            }
        }

        for (_, trail_head) in &mut self.trail_heads {
            let mut completed = false;
            while !completed {
                completed = true;
                for (_idx, trail) in trail_head.into_iter().enumerate() {
                    let last_pos = trail.positions[trail.positions.len() - 1];

                    if last_pos.1 == 9 {
                        continue;
                    }

                    // let x = self.find_next_pos(last_pos.0);

                    // if x.len() == 0 {
                    //     trail_head.remove(idx);
                    //     break;
                    // }

                    // if x.len() >= 1 {
                    //     &trail
                    //         .positions
                    //         .push((x[0], *self.heights.get(&x[0]).unwrap()));
                    // }
                }
            }
        }
    }

    fn find_next_pos(&self, pos: Pos) -> Vec<Pos> {
        let directions: Vec<Vec<i32>> = vec![vec![0, -1], vec![-1, 0], vec![0, 1], vec![1, 0]];
        let mut result = vec![];
        let height = self.heights.get(&pos).unwrap();

        for dir in directions {
            let row_delta = dir[0];
            let col_delta = dir[1];

            if pos.row == 0 && row_delta < 0
                || pos.row == self.rows - 1 && row_delta > 0
                || pos.col == 0 && col_delta < 0
                || pos.col == self.cols - 1 && col_delta > 0
            {
                continue;
            }

            let next_pos = Pos::new(
                (pos.row as i32 + row_delta) as usize,
                (pos.col as i32 + col_delta) as usize,
            );
            if self.heights.get(&next_pos).unwrap() > height {
                result.push(next_pos);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "36");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "17");
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
