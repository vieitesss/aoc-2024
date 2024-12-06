pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub trait Solution {
    fn parse_input(&mut self);

    fn part1(&mut self) -> u64 {
        0
    }

    fn part2(&mut self) -> u64 {
        0
    }
}

pub fn run_day<T: Solution + ?Sized>(day: &mut T) {
    day.parse_input();
    let sol1 = day.part1();
    eprintln!("part 1: {}", sol1);
    let sol2 = day.part2();
    eprintln!("part 2: {}", sol2);
}
