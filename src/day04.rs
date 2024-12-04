use crate::util::Day;

const DAY_NR: u8 = 4;
const PROBLEM_TITLE: &str = "Ceres Search";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let xmas = "XMAS".as_bytes().to_vec();
    let direction = [
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
    ];

    let words = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let mut result = 0;

    for x in 0..words.len() {
        for y in 0..words[x].len() {
            for dir in direction {
                let mut found = true;
                let mut search_x: i32 = x as i32;
                let mut search_y: i32 = y as i32;

                for ch in &xmas {
                    if search_x >= 0
                        && search_x < words.len() as i32
                        && search_y >= 0
                        && search_y < words[x].len() as i32
                    {
                        if words[search_x as usize][search_y as usize] != *ch {
                            found = false;
                            break;
                        } else {
                            search_x += dir[0];
                            search_y += dir[1];
                        }
                    } else {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

fn solve_part2(input: &str) -> String {
    let mas = "MAS".as_bytes().to_vec();
    let sam = "SAM".as_bytes().to_vec();

    let words = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let mut result = 0;

    for x in 1..words.len() - 1 {
        for y in 1..words[x].len() - 1 {
            let x1 = vec![words[x - 1][y - 1], words[x][y], words[x + 1][y + 1]];
            let x2 = vec![words[x - 1][y + 1], words[x][y], words[x + 1][y - 1]];
            if (x1 == mas || x1 == sam) && (x2 == mas || x2 == sam) {
                result += 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_examples() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve_part1(input), "18");
    }

    #[test]
    fn test_part1_with_simple_example() {
        let input = "MMXMASM
MMMMMMM
MSAMXAA";
        assert_eq!(solve_part1(input), "2");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "2483");
    }

    #[test]
    fn test_part2_with_examples() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve_part2(input), "9");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "1925");
    }
}
