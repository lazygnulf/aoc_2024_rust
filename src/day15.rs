use glam::{IVec2, UVec2};

use crate::util::Day;

const DAY_NR: u8 = 15;
const PROBLEM_TITLE: &str = "Warehouse Woes";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut warehouse = Warehouse::new(false, input);
    println!("{:?}", warehouse);
    warehouse.print_map();
    warehouse.exec_moves();

    warehouse.gps_sum().to_string()
}

fn solve_part2(input: &str) -> String {
    let mut warehouse = Warehouse::new(true, input);
    warehouse.print_map();

    "42".to_string()
}

const WALL: char = '#';
const BOX: char = 'O';
const BOX_OPEN: char = '[';
const BOX_CLOSE: char = ']';
const ROBOT: char = '@';
const EMPTY: char = '.';

const UP: IVec2 = IVec2 { x: 0, y: -1 };
const DOWN: IVec2 = IVec2 { x: 0, y: 1 };
const LEFT: IVec2 = IVec2 { x: -1, y: 0 };
const RIGHT: IVec2 = IVec2 { x: 1, y: 0 };

#[derive(Debug)]
struct Warehouse {
    large_boxes: bool,
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    robot_pos: UVec2,
    moves: Vec<IVec2>,
}

impl Warehouse {
    fn new(large_boxes: bool, input: &str) -> Self {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        // read warehouse map
        let mut map = vec![];
        for line in parts[0].lines() {
            let mut map_line: Vec<char> = vec![];
            for ch in line.chars() {
                match ch {
                    WALL => {
                        map_line.push(WALL);
                        if large_boxes {
                            map_line.push(WALL);
                        }
                    }
                    BOX => {
                        if large_boxes {
                            map_line.push(BOX_OPEN);
                            map_line.push(BOX_CLOSE);
                        } else {
                            map_line.push(BOX);
                        }
                    }
                    EMPTY => {
                        map_line.push(EMPTY);
                        if large_boxes {
                            map_line.push(EMPTY);
                        }
                    }
                    ROBOT => {
                        map_line.push(ROBOT);
                        if large_boxes {
                            map_line.push(EMPTY);
                        }
                    }
                    _ => panic!("Unexpected map character."),
                }
            }
            map.push(map_line);
        }

        let height = map.len();
        let width = map[0].len();

        // check map and find robot
        let mut robot_pos: Option<UVec2> = None;
        for y in 0..height {
            for x in 0..width {
                let tile = map[y][x];
                if tile == ROBOT {
                    robot_pos = Some(UVec2 {
                        x: x as u32,
                        y: y as u32,
                    })
                }
            }
        }
        assert_ne!(robot_pos, None);

        // read robot moves
        let mut moves: Vec<IVec2> = vec![];
        for line in parts[1].lines() {
            for ch in line.chars() {
                moves.push(match ch {
                    '^' => UP,
                    'v' => DOWN,
                    '<' => LEFT,
                    '>' => RIGHT,
                    _ => panic!("Unexpected move."),
                });
            }
        }

        Warehouse {
            large_boxes,
            map,
            width: width,
            height: height,
            robot_pos: robot_pos.unwrap(),
            moves,
        }
    }

    fn exec_moves(&mut self) {
        for mv in self.moves.clone() {
            self.exec_move(mv);
            self.print_map();
        }
    }

    fn exec_move(&mut self, dir: IVec2) {
        let new_pos = UVec2 {
            x: (self.robot_pos.x as i32 + dir.x) as u32,
            y: (self.robot_pos.y as i32 + dir.y) as u32,
        };

        if self.is_wall(new_pos) {
            return;
        } else if self.is_box(new_pos) {
            let mut search_pos = new_pos.clone();
            while self.is_box(search_pos) {
                search_pos.x = (search_pos.x as i32 + dir.x) as u32;
                search_pos.y = (search_pos.y as i32 + dir.y) as u32;
            }
            if self.is_empty(search_pos) {
                self.move_box(new_pos, search_pos);
            }
        }

        if self.is_empty(new_pos) {
            self.move_robot(new_pos);
        }
    }

    fn move_robot(&mut self, new_pos: UVec2) {
        assert!(self.is_empty(new_pos));
        self.map[self.robot_pos.y as usize][self.robot_pos.x as usize] = EMPTY;
        self.map[new_pos.y as usize][new_pos.x as usize] = ROBOT;
        self.robot_pos = new_pos;
    }

    fn move_box(&mut self, from: UVec2, to: UVec2) {
        assert!(self.is_box(from));
        assert!(self.is_empty(to));
        self.map[from.y as usize][from.x as usize] = EMPTY;
        self.map[to.y as usize][to.x as usize] = BOX;
    }

    fn gps_sum(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut pos: UVec2 = UVec2::ZERO;

        for y in 0..self.height {
            for x in 0..self.width {
                pos.x = x as u32;
                pos.y = y as u32;
                if self.is_box_open(pos) {
                    sum += 100 * y as u64 + x as u64;
                }
            }
        }
        sum
    }

    fn get_tile(&self, pos: UVec2) -> char {
        self.map[pos.y as usize][pos.x as usize]
    }

    fn is_wall(&self, pos: UVec2) -> bool {
        self.get_tile(pos) == WALL
    }

    fn is_box(&self, pos: UVec2) -> bool {
        match self.large_boxes {
            true => self.get_tile(pos) == BOX_OPEN || self.get_tile(pos) == BOX_CLOSE,
            false => self.get_tile(pos) == BOX,
        }
    }

    fn is_box_open(&self, pos: UVec2) -> bool {
        self.get_tile(pos) == BOX || self.get_tile(pos) == BOX_OPEN
    }

    fn is_empty(&self, pos: UVec2) -> bool {
        self.get_tile(pos) == EMPTY
    }

    fn print_map(&self) {
        let mut pos: UVec2 = UVec2::ZERO;

        println!("Robot @ {}", self.robot_pos);
        for y in 0..self.height {
            for x in 0..self.width {
                pos.x = x as u32;
                pos.y = y as u32;
                print!("{}", self.get_tile(pos));
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example1() -> &'static str {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
    }

    fn example2() -> &'static str {
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example1()), "2028");
        assert_eq!(solve_part1(example2()), "10092");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "1499739");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example2()), "9021");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "42");
    }
}
