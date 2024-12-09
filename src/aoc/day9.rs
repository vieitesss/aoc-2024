use super::Solution;
use std::fs;

#[derive(Default)]
pub struct Day9 {
    files: Vec<usize>,
    free: Vec<usize>,
    free_ocupied: Vec<usize>,
}

impl Day9 {
    fn parse(&mut self, data: &str) {
        for c in data.chars().step_by(2) {
            if let Some(n) = c.to_digit(10) {
                self.files.push(n as usize);
            }
        }
        for c in data.chars().skip(1).step_by(2) {
            if let Some(n) = c.to_digit(10) {
                self.free.push(n as usize);
            }
        }
    }

    fn move_blocks(&mut self) {
        let mut free_len = self.free.len();
        let mut free = 0;
        let mut file = self.files.len() - 1;
        let mut cuantity = self.files[file];
        let mut breaking = false;
        while free < free_len {
            let f = self.free[free];
            for _ in 0..f {
                if cuantity == 0 {
                    self.files.pop();
                    self.free.pop();
                    if free == free_len - 1 {
                        breaking = true;
                        break;
                    }
                    file -= 1;
                    free_len -= 1;
                    cuantity = self.files[file];
                }
                self.free_ocupied.push(file);
                cuantity -= 1;
            }
            if breaking {
                break;
            }
            if cuantity == 0 {
                self.files.pop();
                self.free.pop();
                file -= 1;
                free_len -= 1;
                cuantity = self.files[file];
            }

            free += 1;
        }
        if !breaking {
            self.files[file] = cuantity;
        }
    }

    fn get_checksum(&self) -> usize {
        let mut file = 0;
        let mut free = 0;
        let mut ocupied = 0;
        let mut file_turn = true;
        let mut index = 0;
        let mut sum = 0;

        loop {
            if file == self.files.len() {
                file_turn = false;
            }

            if file_turn {
                for _ in 0..self.files[file] {
                    sum += file * index;
                    index += 1;
                }
                file += 1;
                file_turn = false;
            } else {
                if free < self.free.len() {
                    for _ in 0..self.free[free] {
                        sum += self.free_ocupied[ocupied] * index;
                        index += 1;
                        ocupied += 1;
                    }
                    free += 1;
                    file_turn = file < self.files.len();
                } else {
                    if ocupied < self.free_ocupied.len() {
                        for i in &self.free_ocupied[ocupied..] {
                            sum += *i * index;
                            index += 1;
                        }
                    }
                    break;
                }
            }
        }

        sum
    }

    fn compact_files(&mut self) -> usize {
        self.move_blocks();
        self.get_checksum()
    }
}

impl Solution for Day9 {
    fn parse_input(&mut self) {
        self.parse(&fs::read_to_string("./input/day9").unwrap());
    }

    fn part1(&mut self) -> u64 {
        self.compact_files() as u64
    }

    fn part2(&mut self) -> u64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day9 {
        fn new(data: &str) -> Day9 {
            let mut day = Day9::default();
            day.parse(data);

            day
        }
    }

    #[test]
    fn day9_part1_moveblocks() {
        let mut day = Day9::new(&fs::read_to_string("./example/day9").unwrap());
        day.move_blocks();
        assert_eq!(day.free_ocupied, vec![9, 9, 8, 8, 8, 8, 7, 7, 7, 6, 6, 6]);
        assert_eq!(day.files, vec![2, 3, 1, 3, 2, 4, 1]);
        assert_eq!(day.free, vec![3, 3, 3, 1, 1, 1]);
    }

    #[test]
    fn day9_part1_moveblocks2() {
        let mut day = Day9::new("12345");
        day.move_blocks();
        assert_eq!(day.free_ocupied, vec![2, 2, 2, 2, 2]);
        assert_eq!(day.files, vec![1, 3]);
        assert_eq!(day.free, vec![2]);
    }

    #[test]
    fn day9_part1_sum() {
        let mut day = Day9::new("12345");
        let sum = day.compact_files();
        assert_eq!(sum, 60);
    }

    #[test]
    fn day9_part1_example() {
        let mut day = Day9::new(&fs::read_to_string("./example/day9").unwrap());
        let sum = day.compact_files();
        assert_eq!(sum, 1928);
    }
}
