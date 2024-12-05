use super::Solution;
use std::fs;

#[derive(Default)]
pub struct Day5 {}

impl Day5 {
    fn new() -> Day5 {
        Day5 {}
    }
}

impl Solution for Day5 {
    fn parse_input(&mut self) {
        let _data = fs::read_to_string("./input/day5");
    }

    fn part1(&mut self) -> u64 {
        0
    }

    fn part2(&mut self) -> u64 {
        0
    }
}
