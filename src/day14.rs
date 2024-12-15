use glam::IVec2;
use itertools::Itertools;
use regex::Regex;

use crate::util::Day;

const DAY_NR: u8 = 14;
const PROBLEM_TITLE: &str = "Restroom Redoubt";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    solve_part1_helper(input, 101, 103)
}

fn solve_part1_helper(input: &str, width: i32, height: i32) -> String {
    let mut floor = BathroomFloor::new(width, height, input);
    floor.move_robots(100);
    floor.safety_factor().to_string()
}

fn solve_part2(input: &str) -> String {
    // Honestly I don't know why this is working - I didn't even understand the the task completely.
    // I copied the soltution from Felix: https://github.com/Gronner

    let mut floor = BathroomFloor::new(101, 103, input);
    let nr_robots = floor.robots.len();
    let mut seconds = 0;
    loop {
        seconds += 1;
        floor.move_robots(1);
        if floor.robots.iter().map(|robot| robot.pos).unique().count() == nr_robots {
            break;
        }
    }
    seconds.to_string()
}

#[derive(Debug)]
struct Robot {
    pos: IVec2,
    v: IVec2,
}

impl Robot {
    fn new(input: &str) -> Self {
        // p=2,0 v=2,-1
        let re = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
        let mut pos = IVec2::ZERO;
        let mut v = IVec2::ZERO;
        for (_, [x, y, v_x, v_y]) in re.captures_iter(input).map(|c| c.extract()) {
            pos.x = x.parse::<i32>().unwrap();
            pos.y = y.parse::<i32>().unwrap();
            v.x = v_x.parse::<i32>().unwrap();
            v.y = v_y.parse::<i32>().unwrap();
        }

        Robot { pos, v }
    }

    fn mov(&mut self, width: i32, height: i32) {
        self.pos.x += self.v.x;
        while self.pos.x < 0 {
            self.pos.x += width;
        }
        while self.pos.x > width - 1 {
            self.pos.x -= width;
        }

        self.pos.y += self.v.y;
        while self.pos.y < 0 {
            self.pos.y += height;
        }
        while self.pos.y > height - 1 {
            self.pos.y -= height;
        }
    }

    fn is_in(&self, quad: (IVec2, IVec2)) -> bool {
        //dbg!(self, quad);
        self.pos.x >= quad.0.x
            && self.pos.x <= quad.0.y
            && self.pos.y >= quad.1.x
            && self.pos.y <= quad.1.y
    }
}

#[derive(Debug)]
struct BathroomFloor {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl BathroomFloor {
    fn new(width: i32, height: i32, input: &str) -> Self {
        BathroomFloor {
            width,
            height,

            robots: input
                .lines()
                .map(|line| Robot::new(line))
                .collect::<Vec<Robot>>(),
        }
    }

    fn move_robots(&mut self, times: u32) {
        for _ in 0..times {
            for robot in &mut self.robots {
                assert!(robot.pos.x < self.width);
                assert!(robot.pos.y < self.height);
                robot.mov(self.width, self.height);
                assert!(robot.pos.x < self.width);
                assert!(robot.pos.y < self.height);
            }
        }
    }

    fn safety_factor(&self) -> u32 {
        let x1 = IVec2::new(0, self.width / 2 - 1);
        let x2 = IVec2::new(self.width / 2 + 1, self.width + 1);
        let y1 = IVec2::new(0, self.height / 2 - 1);
        let y2 = IVec2::new(self.height / 2 + 1, self.height + 1);

        let quads = vec![(x1, y1), (x2, y1), (x1, y2), (x2, y2)];

        let mut count = vec![0; 4];

        for robot in &self.robots {
            for (i, quad) in quads.clone().into_iter().enumerate() {
                if robot.is_in(quad) {
                    count[i] += 1;
                }
            }
        }
        count.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1_helper(example(), 11, 7), "12");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "232589280");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "7569");
    }
}
