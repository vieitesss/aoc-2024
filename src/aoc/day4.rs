use crate::utils::{
    dir::*,
    matrix::{Matrix, MatrixTrait},
    parser,
};

use super::Solution;
use std::fs;

#[derive(Default)]
pub struct Day4 {
    matrix: Matrix<char>,
}

impl Day4 {
    fn find_next(&self, pos: (isize, isize), dir: &Dir, find: char) -> bool {
        let checking = dir.from(pos);

        if let Some(c) = self.matrix.from_pos(checking) {
            if *c != find {
                return false;
            }

            match find {
                'M' => return self.find_next(checking, dir, 'A'),
                'A' => return self.find_next(checking, dir, 'S'),
                'S' => return true,
                _ => panic!("Should not check for {find}"),
            };
        }

        false
    }

    fn is_xmas(&self, pos: (isize, isize), dir: &Dir) -> bool {
        if self.matrix.from_pos(pos) != Some('X').as_ref() {
            return false;
        }

        self.find_next(pos, dir, 'M')
    }

    fn count_xmas(&self) -> usize {
        let xs = self.matrix.positions(&'X');
        xs.iter().fold(0, |acc, &pos| {
            let current = (pos.0 as isize, pos.1 as isize);
            acc + DIRS.iter().filter(|dir| self.is_xmas(current, dir)).count()
        })
    }

    fn count_x_mas(&self) -> usize {
        let ass = self.matrix.positions(&'A');
        ass.iter()
            .filter(|&&pos| {
                let current = (pos.0 as isize, pos.1 as isize);
                let topleft = self.matrix.from_pos(Dir::TopLeft.from(current));
                let bottomright = self.matrix.from_pos(Dir::BottomRight.from(current));

                if !matches!(
                    (topleft, bottomright),
                    (Some('S'), Some('M')) | (Some('M'), Some('S'))
                ) {
                    return false;
                }

                let topright = self.matrix.from_pos(Dir::TopRight.from(current));
                let bottomleft = self.matrix.from_pos(Dir::BottomLeft.from(current));

                matches!(
                    (topright, bottomleft),
                    (Some('S'), Some('M')) | (Some('M'), Some('S'))
                )
            })
            .count()
    }
}

impl Solution for Day4 {
    fn parse_input(&mut self) {
        let data = fs::read_to_string("./input/day4").unwrap();
        self.matrix = parser::to_chars_matrix(&data);
    }

    fn part1(&mut self) -> u64 {
        self.count_xmas() as u64
    }

    fn part2(&mut self) -> u64 {
        self.count_x_mas() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day4 {
        fn new(matrix: Matrix<char>) -> Day4 {
            Day4 { matrix }
        }
    }

    #[test]
    fn day4_part1_is_xmas() {
        let data = fs::read_to_string("./example/day4").unwrap();
        let day = Day4::new(parser::to_chars_matrix(&data));
        assert!(day.is_xmas((0, 5), &Dir::Right));
        assert!(day.is_xmas((0, 4), &Dir::BottomRight));
    }

    #[test]
    fn day4_part1_is_not_xmas() {
        let data = fs::read_to_string("./example/day4").unwrap();
        let day = Day4::new(parser::to_chars_matrix(&data));
        assert!(!day.is_xmas((2, 2), &Dir::Left));
        assert!(!day.is_xmas((7, 2), &Dir::BottomRight));
    }

    #[test]
    fn day4_part1_example_count() {
        let data = fs::read_to_string("./example/day4").unwrap();
        let day = Day4::new(parser::to_chars_matrix(&data));
        assert_eq!(day.count_xmas(), 18);
    }

    #[test]
    fn day4_part2_example_count() {
        let data = fs::read_to_string("./example/day4").unwrap();
        let day = Day4::new(parser::to_chars_matrix(&data));
        assert_eq!(day.count_x_mas(), 9);
    }
}
