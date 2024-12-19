use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::partition;
use owo_colors::OwoColorize;
use pathfinding::prelude::{astar, astar_bag, astar_bag_collect, dijkstra};

use crate::util::Day;

const DAY_NR: u8 = 16;
const PROBLEM_TITLE: &str = "Reindeer Maze";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut maze = Maze::new(input);
    if cfg!(test) {
        maze.print(true);
    }
    let score = maze.best_path_score();
    if cfg!(test) {
        maze.print(true);
    }

    score.to_string()
}

fn solve_part2(input: &str) -> String {
    let mut maze = Maze::new(input);
    if cfg!(test) {
        maze.print(false);
    }
    let tiles = maze.nr_best_path_tiles();
    if cfg!(test) {
        maze.print(false);
    }

    tiles.to_string()
}

// type Pos = IVec2;

// const DIRECTIONS: [Pos; 4] = [
//     Pos::NEG_Y, // up
//     Pos::X,     // right
//     Pos::Y,     // down
//     Pos::NEG_X, // left
// ];

// enum Direction {
//     UP = 0,
//     RIGHT = 1,
//     DOWN = 2,
//     LEFT = 3,
// }

// #[derive(Debug, Eq, Hash)]
// struct DirectedPos {
//     pos: Pos,
//     dir: Direction,
// }

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    start: IVec2,
    end: IVec2,
    obstacles: HashSet<IVec2>,
    path: Option<HashMap<IVec2, IVec2>>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut width = input.lines().next().unwrap().len();
        let mut height = 0;
        let mut start: Option<IVec2> = None;
        let mut end: Option<IVec2> = None;
        let mut obstacles: HashSet<IVec2> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                match ch {
                    '#' => {
                        obstacles.insert(pos);
                    }
                    '.' => (),
                    'S' => {
                        start = Some(pos);
                    }
                    'E' => {
                        end = Some(pos);
                    }
                    _ => panic!("unexpected maze character {}", ch),
                };
            }

            height += 1;
        }
        Maze {
            width,
            height,
            obstacles,
            start: start.expect("No Start found."),
            end: end.expect("No End found."),
            path: None,
        }
    }

    fn best_path_score(&mut self) -> usize {
        let result = astar(
            &(self.start, IVec2::X),
            |(pos, dir)| {
                let next_pos = pos + dir;
                if self.obstacles.contains(&next_pos) {
                    vec![((*pos, dir.perp()), 1000), ((*pos, -dir.perp()), 1000)]
                } else {
                    vec![
                        ((next_pos, *dir), 1),
                        ((*pos, dir.perp()), 1000),
                        ((*pos, -dir.perp()), 1000),
                    ]
                }
            },
            |(_, _)| 0,
            |&(pos, _)| pos == self.end,
        );

        self.path = Some(HashMap::from_iter(result.as_ref().unwrap().0.clone()));

        result.unwrap().1
    }

    fn nr_best_path_tiles(&mut self) -> usize {
        let result = astar_bag(
            &(self.start, IVec2::X),
            |(pos, dir)| {
                let next_pos = pos + dir;
                if self.obstacles.contains(&next_pos) {
                    vec![((*pos, dir.perp()), 1000), ((*pos, -dir.perp()), 1000)]
                } else {
                    vec![
                        ((next_pos, *dir), 1),
                        ((*pos, dir.perp()), 1000),
                        ((*pos, -dir.perp()), 1000),
                    ]
                }
            },
            |(_, _)| 0,
            |&(pos, _)| pos == self.end,
        );

        let mut path = HashMap::new();
        for p in result.unwrap().0 {
            for x in p {
                path.insert(x.0, x.1);
            }
        }

        self.path = Some(path.clone());

        path.len()
    }

    fn print(&self, with_direction: bool) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                if self.obstacles.contains(&pos) {
                    print!("{}", "#".red())
                } else if self.start == pos {
                    print!("{}", "S".yellow());
                } else if self.end == pos {
                    print!("{}", "E".green());
                } else if let Some(path) = &self.path {
                    if path.contains_key(&pos) {
                        if with_direction {
                            match path.get(&pos).unwrap() {
                                &IVec2::X => print!("{}", '>'.green()),
                                &IVec2::NEG_X => print!("{}", '<'.green()),
                                &IVec2::Y => print!("{}", 'v'.green()),
                                &IVec2::NEG_Y => print!("{}", '^'.green()),

                                _ => panic!("unexpected direction"),
                            }
                        } else {
                            print!("{}", 'O'.green());
                        }
                    } else {
                        print!("{}", ".".white());
                    }
                } else {
                    print!("{}", ".".white());
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
    }

    fn example2() -> &'static str {
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "7036");
        assert_eq!(solve_part1(example2()), "11048");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "109516");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "45");
        assert_eq!(solve_part2(example2()), "64");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "568");
    }
}
