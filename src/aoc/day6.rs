use super::Solution;
use crate::utils::{
    dir::Dir,
    matrix::{Matrix, MatrixTrait},
    parser,
};
use std::{collections::HashMap, fs};

type Point = (usize, usize);

#[derive(Default)]
pub struct Day6 {
    table: Matrix<char>,
    dirs: HashMap<Point, Vec<Dir>>,
    start: Point,
    current: Point,
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

    fn get_next(&mut self) -> Point {
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

    fn reset(&mut self) {
        self.dir = Dir::Top;
        self.current = self.start;
        self.dirs = HashMap::new();
    }

    fn is_loop(&mut self, p: Point) -> bool {
        if p == self.start {
            return false;
        }

        self.table[p.0][p.1] = '#';

        while !self.is_edge_current() {
            self.current = self.get_next();
            if let Some(d) = self.dirs.get(&self.current) {
                if d.contains(&self.dir) {
                    self.table[p.0][p.1] = 'X';
                    return true;
                } else {
                    let updated = &d[..];
                    self.dirs.insert(self.current, updated.to_vec());
                }
            } else {
                self.dirs.insert(self.current, vec![self.dir]);
            }
        }

        if let Some(d) = self.dirs.get(&p) {
            if d.contains(&self.dir) {
                self.table[p.0][p.1] = 'X';
                return true;
            }
        }

        self.table[p.0][p.1] = 'X';
        false
    }
}

impl Solution for Day6 {
    fn parse_input(&mut self) {
        self.table = parser::to_chars_matrix(&fs::read_to_string("./input/day6").unwrap());
        self.dir = Dir::Top;
        self.start = self.table.positions(&'^')[0];
        self.current = self.start;
    }

    fn part1(&mut self) -> u64 {
        while !self.is_edge_current() {
            self.table[self.current.0][self.current.1] = 'X';
            self.current = self.get_next();
        }

        self.table[self.current.0][self.current.1] = 'X';

        self.table.positions(&'X').len() as u64
    }

    fn part2(&mut self) -> u64 {
        let visited = self.table.positions(&'X');
        visited
            .iter()
            .filter(|&p| {
                self.reset();
                self.is_loop(*p)
            })
            .count() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day6 {
        fn new() -> Day6 {
            let mut day = Day6::default();

            day.table = parser::to_chars_matrix(&fs::read_to_string("./example/day6").unwrap());
            day.start = day.table.positions(&'^')[0];
            day.current = day.start;

            day
        }

        fn part1(&mut self) {
            while !self.is_edge_current() {
                self.table[self.current.0][self.current.1] = 'X';
                self.current = self.get_next();
            }

            self.table[self.current.0][self.current.1] = 'X';
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
        day.part1();
        assert_eq!(day.table.positions(&'X').len(), 41)
    }

    #[test]
    fn day6_part2_example() {
        let mut day = Day6::new();
        day.part1();
        let _visited = day.table.positions(&'X');
        day.reset();
        assert!(day.is_loop((6, 3)));
        day.reset();
        assert!(day.is_loop((7, 6)));
        day.reset();
        assert!(day.is_loop((7, 7)));
        day.reset();
        assert!(day.is_loop((8, 1)));
        day.reset();
        assert!(day.is_loop((8, 3)));
        day.reset();
        assert!(day.is_loop((9, 7)));
        day.reset();
        assert!(!day.is_loop((1, 7)));
        let visited = day.table.positions(&'X');
        let total = visited
            .iter()
            .filter(|&p| {
                day.reset();
                day.is_loop(*p)
            })
            .count();
        assert_eq!(total, 6);
    }
}
