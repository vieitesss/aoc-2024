use std::time::Duration;
use std::time::Instant;

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
    let start = Instant::now();
    day.parse_input();
    let parse_time = start.elapsed().as_millis();
    println!("{:3}.{:03} Parsing", parse_time / 1000, parse_time % 1000);

    let start = Instant::now();
    let p1 = day.part1();
    let p1_time = start.elapsed();
    print_solution(1, p1, p1_time);

    let start = Instant::now();
    let p2 = day.part2();
    let p2_time = start.elapsed();
    print_solution(2, p2, p2_time);
}

fn print_solution(which: usize, output: u64, duration: Duration) {
    let ms = duration.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;

    println!("{sec_part:3}.{ms_part:03} Part {which}: {}", output);
}
