mod aoc;
mod utils;

use aoc::day2::Day2;
use aoc::day3::Day3;
use aoc::day4::Day4;
use aoc::day5::Day5;
use aoc::{day1::Day1, Solution};
use std::{env, io, process::exit};

fn usage_exit(day: &str) {
    eprintln!("Usage: {} <day>", day);
    eprintln!("\n\t- day: number between 1 and 25");
    exit(1);
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage_exit(&args[0]);
    }

    let day: usize = args[1].parse().unwrap();

    if day < 1 || day > 25 {
        usage_exit(&args[0]);
    }

    let mut day1 = Day1::default();
    let mut day2 = Day2::default();
    let mut day3 = Day3::default();
    let mut day4 = Day4::default();
    let mut day5 = Day5::default();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day1,
        &mut day2,
        &mut day3,
        &mut day4,
        &mut day5,
    ];

    let d = &mut days[day - 1];
    aoc::run_day(*d);

    Ok(())
}
