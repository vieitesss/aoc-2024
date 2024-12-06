use super::Solution;
use crate::utils::{
    dir::Dir,
    matrix::{Matrix, MatrixTrait},
    parser,
};
use std::fs;

#[derive(Default)]
pub struct Day6 {
    table: Matrix<char>,
    current: (usize, usize),
    dir: Dir,
}

impl Day6 {
    fn is_wall(&mut self, p: (isize, isize)) -> bool {
        self.table.from_pos(p) == Some(&'#')
    }

    fn update_dir(&mut self) {
        match self.dir {
            Dir::Top => self.dir = Dir::Right,
            Dir::Right => self.dir = Dir::Bottom,
            Dir::Left => self.dir = Dir::Top,
            Dir::Bottom => self.dir = Dir::Left,
            _ => panic!("Not a valid dir {:?}", self.dir),
        }
    }

    fn get_next(&mut self) -> (usize, usize) {
        let current = (self.current.0 as isize, self.current.1 as isize);
        let mut next = self.dir.from(current);

        while self.is_wall(next) {
            self.update_dir();
            next = self.dir.from(current);
        }

        (next.0 as usize, next.1 as usize)
    }

    fn is_edge_current(&self) -> bool {
        self.current.0 == 0
            || self.current.0 as usize == self.table.len() - 1
            || self.current.1 == 0
            || self.current.1 as usize == self.table[0].len() - 1
    }
}

impl Solution for Day6 {
    fn parse_input(&mut self) {
        self.table = parser::to_chars_matrix(&fs::read_to_string("./input/day6").unwrap());
        self.dir = Dir::Top;
        self.current = self.table.positions(&'^')[0]
    }

    fn part1(&mut self) -> u64 {
        let mut table = self.table.clone();

        while !self.is_edge_current() {
            table[self.current.0][self.current.1] = 'X';
            self.current = self.get_next();
        }

        table[self.current.0][self.current.1] = 'X';

        table.positions(&'X').len() as u64
    }

    fn part2(&mut self) -> u64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day6 {
        fn new() -> Day6 {
            let mut day = Day6::default();

            day.table = parser::to_chars_matrix(&fs::read_to_string("./example/day6").unwrap());
            day.current = day.table.positions(&'^')[0];

            day
        }
    }

    #[test]
    fn day6_part1_parse() {
        let day = Day6::new();
        assert_eq!(day.table.len(), 10);
        assert_eq!(day.table[0].len(), 10);
        assert_eq!(day.current, (6, 4));
        assert_eq!(day.dir, Dir::Top);
    }

    #[test]
    fn day6_part1_example() {
        let mut day = Day6::new();

        while !day.is_edge_current() {
            day.table[day.current.0][day.current.1] = 'X';
            day.current = day.get_next();
        }

        day.table[day.current.0][day.current.1] = 'X';

        assert_eq!(day.table.positions(&'X').len(), 41)
    }
}
