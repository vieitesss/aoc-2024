pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::{io, path::Path};

pub trait Solution {
    type Item;

    fn parse_input<P>(path: P) -> io::Result<impl Iterator<Item = Self::Item>>
    where
        P: AsRef<Path>;

    fn part1() -> u64 {
        todo!("not implemented");
    }

    fn part2() -> u64 {
        todo!("not implemented");
    }
}
