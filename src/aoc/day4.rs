use crate::utils::{
    dir::*,
    matrix::{Matrix, MatrixTrait, Row},
    parser,
};

use super::Solution;
use std::{fs, io, path::Path};

pub struct Day4 {
    matrix: Matrix<char>,
}

impl Day4 {
    fn new(matrix: Matrix<char>) -> Day4 {
        Day4 { matrix }
    }

    fn find_next(&self, pos: (isize, isize), dir: &Dir, find: char) -> bool {
        let checking = dir.from(pos);

        if let Some(c) = self.matrix.get_pos(checking) {
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
        if self.matrix.get_pos(pos) != Some('X').as_ref() {
            return false;
        }

        self.find_next(pos, dir, 'M')
    }

    fn count_xmas(&self) -> usize {
        let xs = self.matrix.find_element(&'X');
        xs.iter().fold(0, |acc, &pos| {
            let current = (pos.0 as isize, pos.1 as isize);
            acc + DIRS.iter().filter(|dir| self.is_xmas(current, dir)).count()
        })
    }

    fn count_x_mas(&self) -> usize {
        let ass = self.matrix.find_element(&'A');
        ass.iter()
            .filter(|&&pos| {
                let current = (pos.0 as isize, pos.1 as isize);
                let topleft = self.matrix.get_pos(Dir::TopLeft.from(current));
                let bottomright = self.matrix.get_pos(Dir::BottomRight.from(current));

                if !matches!(
                    (topleft, bottomright),
                    (Some('S'), Some('M')) | (Some('M'), Some('S'))
                ) {
                    return false;
                }

                let topright = self.matrix.get_pos(Dir::TopRight.from(current));
                let bottomleft = self.matrix.get_pos(Dir::BottomLeft.from(current));

                matches!(
                    (topright, bottomleft),
                    (Some('S'), Some('M')) | (Some('M'), Some('S'))
                )
            })
            .count()
    }
}

impl Solution for Day4 {
    type Item = Row<char>;

    fn parse_input<P>(path: P) -> io::Result<impl Iterator<Item = Self::Item>>
    where
        P: AsRef<Path>,
    {
        let data = fs::read_to_string(path).unwrap();

        Ok(parser::to_chars_matrix(&data).into_iter())
    }

    fn part1() -> u64 {
        let data = Day4::parse_input("./input/day4").unwrap().collect();
        let day = Day4::new(data);
        day.count_xmas() as u64
    }

    fn part2() -> u64 {
        let data = Day4::parse_input("./input/day4").unwrap().collect();
        let day = Day4::new(data);
        day.count_x_mas() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day4_part1_is_xmas() {
        let data = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert!(day.is_xmas((0, 5), &Dir::Right));
        assert!(day.is_xmas((0, 4), &Dir::BottomRight));
    }

    #[test]
    fn day4_part1_is_not_xmas() {
        let data = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert!(!day.is_xmas((2, 2), &Dir::Left));
        assert!(!day.is_xmas((7, 2), &Dir::BottomRight));
    }

    #[test]
    fn day4_part1_example_count() {
        let data = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert_eq!(day.count_xmas(), 18);
    }

    #[test]
    fn day4_part2_example_count() {
        let data = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert_eq!(day.count_x_mas(), 9);
    }
}
