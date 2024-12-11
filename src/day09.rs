use crate::util::Day;

const DAY_NR: u8 = 9;
const PROBLEM_TITLE: &str = "Disk Fragmenter";

pub fn get_day() -> Day {
    Day::new(DAY_NR, PROBLEM_TITLE, solve_part1, solve_part2)
}

fn solve_part1(input: &str) -> String {
    let mut disk_map = DiskMap::new(input);
    disk_map.create_block_map();
    disk_map.fragment_block_map();
    disk_map.checksum().to_string()
}

fn solve_part2(input: &str) -> String {
    let mut disk_map = DiskMap::new(input);
    disk_map.create_block_map();
    disk_map.defragment_block_map();
    disk_map.checksum().to_string()
}

const FREE: i32 = -1;

#[derive(Debug)]
struct DiskMap {
    compact_map: Vec<u8>,
    block_map: Vec<i32>,
    max_file_id: i32,
}

impl DiskMap {
    fn new(input: &str) -> Self {
        DiskMap {
            compact_map: input
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect(),
            block_map: vec![],
            max_file_id: -1,
        }
    }

    fn create_block_map(&mut self) {
        let mut is_file = true;
        let mut file_id: i32 = 0;
        for n in &self.compact_map {
            if is_file {
                for _ in 0..*n {
                    self.block_map.push(file_id);
                }
                file_id += 1;
            } else {
                for _ in 0..*n {
                    self.block_map.push(FREE);
                }
            }

            is_file = !is_file;
        }
        self.max_file_id = file_id - 1;
    }

    fn fragment_block_map(&mut self) {
        let mut move_to_idx: usize = 0;
        let mut move_from_idx: usize = self.block_map.len() - 1;

        while move_to_idx < move_from_idx {
            // find next free slot from left
            while self.block_map[move_to_idx] != FREE && move_from_idx < self.block_map.len() {
                move_to_idx += 1;
            }

            // find the next file block from right
            while self.block_map[move_from_idx] == FREE && move_from_idx > 0 {
                move_from_idx -= 1;
            }

            // swap if the indexes did not cross over
            if move_to_idx < move_from_idx {
                assert_eq!(self.block_map[move_to_idx], FREE);
                assert_ne!(self.block_map[move_from_idx], FREE);

                self.block_map[move_to_idx] = self.block_map[move_from_idx];
                self.block_map[move_from_idx] = FREE;
            }
        }
    }

    fn defragment_block_map(&mut self) {
        for file_id in (0..=self.max_file_id).rev() {
            let (file_begin_idx, file_end_idx) = self.find_file(file_id);
            let space_need = file_end_idx - file_begin_idx + 1;
            match self.find_leftmost_free_space(space_need) {
                Some(free_begin_idx) => {
                    if free_begin_idx < file_begin_idx {
                        // move file if free space ist found on the left side
                        let mut free_idx = free_begin_idx;
                        for i in file_begin_idx..=file_end_idx {
                            assert_eq!(self.block_map[free_idx], FREE);
                            assert_eq!(self.block_map[i], file_id);
                            self.block_map[free_idx] = self.block_map[i];
                            self.block_map[i] = FREE;
                            free_idx += 1;
                        }
                    }
                }
                None => continue,
            }
        }
    }

    fn find_file(&self, file_id: i32) -> (usize, usize) {
        assert_ne!(file_id, FREE);

        let mut begin_idx = 0;
        let mut end_idx = 0;

        for i in 0..self.block_map.len() {
            if self.block_map[i] == file_id {
                begin_idx = i;
                break;
            }
        }
        assert_eq!(self.block_map[begin_idx], file_id);

        for i in begin_idx..self.block_map.len() {
            if self.block_map[i] == file_id {
                end_idx = i;
            } else {
                break;
            }
        }
        assert_eq!(self.block_map[end_idx], file_id);
        assert!(begin_idx <= end_idx);

        (begin_idx, end_idx)
    }

    fn find_leftmost_free_space(&self, space_need: usize) -> Option<usize> {
        let mut start_idx = 0;
        let mut count = 0;

        for i in 0..self.block_map.len() {
            if self.block_map[i] == FREE {
                if count == 0 {
                    // start of a free range
                    start_idx = i;
                }
                count += 1;
                if count == space_need {
                    // we found a suitable free space
                    return Some(start_idx);
                }
            } else {
                // we hit the end of a free space
                count = 0;
            }
        }

        None
    }

    fn checksum(&self) -> u64 {
        let mut result: u64 = 0;

        for i in 0..self.block_map.len() {
            if self.block_map[i] != FREE {
                result += i as u64 * self.block_map[i] as u64;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> &'static str {
        "2333133121414131402"
    }

    #[test]
    fn test_part1_with_examples() {
        assert_eq!(solve_part1(example()), "1928");
    }

    #[test]
    fn test_part1_with_input() {
        assert_eq!(solve_part1(&get_day().read_input()), "6341711060162");
    }

    #[test]
    fn test_part2_with_examples() {
        assert_eq!(solve_part2(example()), "2858");
    }

    #[test]
    fn test_part2_with_input() {
        assert_eq!(solve_part2(&get_day().read_input()), "6377400869326");
    }
}
