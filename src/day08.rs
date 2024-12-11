use crate::util::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DAY_NR: u8 = 8;
const PROBLEM_TITLE: &str = "Resonant Collinearity";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Pos {
    row: i32,
    col: i32,
}

fn solve_part1(input: &str) -> String {
    let mut map: HashMap<char, HashSet<Pos>> = HashMap::new();

    let mut row = 0;
    for line in input.lines() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                _ => match map.get_mut(&ch) {
                    Some(list) => list.insert(Pos {
                        row,
                        col: col.try_into().unwrap(),
                    }),
                    None => {
                        map.insert(ch, HashSet::new());
                        map.get_mut(&ch).unwrap().insert(Pos {
                            row,
                            col: col.try_into().unwrap(),
                        })
                    }
                },
            };
        }
        row += 1;
    }

    let height = row;
    let width = input.lines().next().unwrap().as_bytes().len() as i32;

    //println!("{:?}", map);
    println!("Height: {} Width: {}", height, width);

    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (freq, positions) in map.iter() {
        println!("Frequency: {}", freq);
        for pair in positions.iter().combinations(2) {
            print!("Pair {:?}: ", pair);
            let anti1 = Pos {
                row: pair[0].row + 2 * (pair[1].row - pair[0].row),
                col: pair[0].col + 2 * (pair[1].col - pair[0].col),
            };
            print!("Anti1: {:?} ", anti1);
            if anti1.row >= 0 && anti1.row < height && anti1.col >= 0 && anti1.col < width {
                antinodes.insert(anti1);
            } else {
                print!("(out) ");
            }
            let anti2 = Pos {
                row: pair[1].row + 2 * (pair[0].row - pair[1].row),
                col: pair[1].col + 2 * (pair[0].col - pair[1].col),
            };
            print!("Anti2: {:?} ", anti2);
            if anti2.row >= 0 && anti2.row < height && anti2.col >= 0 && anti2.col < width {
                antinodes.insert(anti2);
            } else {
                print!("(out) ");
            }
            println!();
        }
        println!();
    }
    println!("Antinodes: {:?}", antinodes);

    for row in 0..height {
        for col in 0..width {
            if antinodes.contains(&Pos { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    antinodes.len().to_string()
}

fn solve_part2(input: &str) -> String {
    let mut map: HashMap<char, HashSet<Pos>> = HashMap::new();

    let mut row = 0;
    for line in input.lines() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                _ => match map.get_mut(&ch) {
                    Some(list) => list.insert(Pos {
                        row,
                        col: col.try_into().unwrap(),
                    }),
                    None => {
                        map.insert(ch, HashSet::new());
                        map.get_mut(&ch).unwrap().insert(Pos {
                            row,
                            col: col.try_into().unwrap(),
                        })
                    }
                },
            };
        }
        row += 1;
    }

    let height = row;
    let width = input.lines().next().unwrap().as_bytes().len() as i32;

    //println!("{:?}", map);
    println!("Height: {} Width: {}", height, width);

    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (freq, positions) in map.iter() {
        println!("Frequency: {}", freq);
        for pair in positions.iter().combinations(2) {
            print!("Pair {:?}: ", pair);

            let row_diff = pair[1].row - pair[0].row;
            let col_diff = pair[1].col - pair[0].col;

            let origin = Pos {
                row: pair[0].row,
                col: pair[0].col,
            };
            antinodes.insert(origin);

            let mut stop = false;
            let mut n = 0;
            while !stop {
                let anti = Pos {
                    row: origin.row + n * row_diff,
                    col: origin.col + n * col_diff,
                };
                print!("Anti: {:?} ", anti);
                if anti.row >= 0 && anti.row < height && anti.col >= 0 && anti.col < width {
                    antinodes.insert(anti);
                } else {
                    print!("(out) ");
                    stop = true;
                }
                println!();
                n += 1;
            }

            n = 0;
            stop = false;
            while !stop {
                let anti = Pos {
                    row: origin.row - n * row_diff,
                    col: origin.col - n * col_diff,
                };
                print!("Anti: {:?} ", anti);
                if anti.row >= 0 && anti.row < height && anti.col >= 0 && anti.col < width {
                    antinodes.insert(anti);
                } else {
                    print!("(out) ");
                    stop = true;
                }
                println!();
                n += 1;
            }
        }
        println!();
    }
    println!("Antinodes: {:?}", antinodes);

    for row in 0..height {
        for col in 0..width {
            if antinodes.contains(&Pos { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    fn example2() -> &'static str {
        "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "14");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "17");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "34");
        assert_eq!(solve_part2(example2()), "9");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
