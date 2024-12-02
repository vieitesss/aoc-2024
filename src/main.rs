mod aoc;

use crate::aoc::Solution;
use aoc::day1::Day1;
use aoc::day2::Day2;
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

    let day = &args[1].parse::<u8>().unwrap();

    if *day < 1 || *day > 25 {
        usage_exit(&args[0]);
    }

    match day {
        1 => eprintln!("part 1: {}\npart 2: {}", Day1::part1(), Day1::part2()),
        2 => eprintln!("part 1: {}\npart 2: {}", Day2::part1(), Day2::part2()),
        _ => eprintln!("Day {} not done yet", day),
    }

    Ok(())
}
