use super::Solution;
use crate::utils::{
    matrix::{Matrix, MatrixTrait},
    parser,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Point = (usize, usize);

#[derive(Default)]
pub struct Day8 {
    table: Matrix<char>,
    antennas: HashMap<char, Vec<Point>>,
}

impl Day8 {
    fn get_antennas(&mut self) {
        for (i, _) in self.table.iter().enumerate() {
            for (j, _) in self.table[i].iter().enumerate() {
                let current = self.table[i][j];
                if current == '.' {
                    continue;
                }
                let positions = self.antennas.entry(current).or_insert_with(Vec::new);
                positions.push((i, j));
            }
        }
    }

    fn get_antinodes(&self) -> usize {
        let mut total = HashSet::new();
        for antennas in self.antennas.values() {
            if antennas.len() < 2 {
                continue;
            }
            for (i, first) in antennas.iter().enumerate() {
                for second in antennas.iter().skip(i + 1) {
                    let (fx, fy) = (first.0 as isize, first.1 as isize);
                    let (sx, sy) = (second.0 as isize, second.1 as isize);

                    let dx = sx - fx;
                    let dy = sy - fy;

                    let antinode1 = (sx + dx, sy + dy);
                    if !self.table.is_outbounds(antinode1) {
                        total.insert(antinode1);
                    }

                    let antinode2 = (fx - dx, fy - dy);
                    if !self.table.is_outbounds(antinode2) {
                        total.insert(antinode2);
                    }
                }
            }
        }

        total.len()
    }

    fn get_antinodes2(&self) -> usize {
        let mut total = HashSet::new();
        for antennas in self.antennas.values() {
            if antennas.len() < 2 {
                continue;
            }
            for (i, first) in antennas.iter().enumerate() {
                let (fx, fy) = (first.0 as isize, first.1 as isize);
                total.insert((fx, fy));
                for second in antennas.iter().skip(i + 1) {
                    let (sx, sy) = (second.0 as isize, second.1 as isize);
                    total.insert((sx, sy));

                    let dx = sx - fx;
                    let dy = sy - fy;

                    let mut antinode1 = (sx + dx, sy + dy);
                    while !self.table.is_outbounds(antinode1) {
                        total.insert(antinode1);
                        antinode1 = (antinode1.0 + dx, antinode1.1 + dy);
                    }

                    let mut antinode2 = (fx - dx, fy - dy);
                    while !self.table.is_outbounds(antinode2) {
                        total.insert(antinode2);
                        antinode2 = (antinode2.0 - dx, antinode2.1 - dy);
                    }
                }
            }
        }

        total.len()
    }
}

impl Solution for Day8 {
    fn parse_input(&mut self) {
        self.table = parser::to_chars_matrix(&fs::read_to_string("./input/day8").unwrap());
        self.get_antennas()
    }

    fn part1(&mut self) -> u64 {
        self.get_antinodes() as u64
    }

    fn part2(&mut self) -> u64 {
        self.get_antinodes2() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day8 {
        fn new() -> Day8 {
            let mut day = Day8::default();
            day.table = parser::to_chars_matrix(&fs::read_to_string("./example/day8").unwrap());
            day.get_antennas();

            day
        }
    }

    #[test]
    fn day8_part1_parse() {
        let day = Day8::new();
        assert_eq!(day.antennas.len(), 2);
        assert!(day
            .antennas
            .get(&'A')
            .is_some_and(|p| *p == vec![(5, 6), (8, 8), (9, 9)]));
        assert!(day
            .antennas
            .get(&'0')
            .is_some_and(|p| *p == vec![(1, 8), (2, 5), (3, 7), (4, 4)]));
    }

    #[test]
    fn day8_part1_example() {
        let day = Day8::new();
        assert_eq!(day.get_antinodes(), 14);
    }

    #[test]
    fn day8_part2_example() {
        let day = Day8::new();
        assert_eq!(day.get_antinodes2(), 34);
    }
}
