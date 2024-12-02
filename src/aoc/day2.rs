#![allow(dead_code)]
use super::Solution;
use std::{fs, io, path::Path};

type Report = Vec<i64>;

#[derive(Default)]
pub struct Day2 {}

#[derive(PartialEq, Debug)]
enum DIR {
    INCREASING,
    DECREASING,
}

impl Day2 {
    fn is_safe_part1(report: &[i64]) -> bool {
        if report.len() < 2 {
            return false;
        }

        let mut dir: Option<DIR> = None;

        for pair in report.windows(2) {
            if !Day2::is_valid_pair(&mut dir, pair[0], pair[1]) {
                return false;
            }
        }

        true
    }

    fn get_dir(current: i64, next: i64) -> DIR {
        if current - next < 0 {
            DIR::INCREASING
        } else {
            DIR::DECREASING
        }
    }

    fn check_diff(current: i64, next: i64) -> bool {
        let diff = next - current;
        let abs_diff = diff.abs();

        if abs_diff == 0 || abs_diff > 3 {
            return false;
        }

        true
    }

    fn is_valid_pair(dir: &mut Option<DIR>, current: i64, next: i64) -> bool {
        if !Day2::check_diff(current, next) {
            return false;
        }

        let current_dir = Day2::get_dir(current, next);

        if let Some(prev_dir) = &dir {
            if prev_dir != &current_dir {
                return false;
            }
        } else {
            *dir = Some(current_dir);
        }

        true
    }

    fn is_valid_report(report: &[i64]) -> (bool, usize) {
        if report.len() < 2 {
            return (false, 0);
        }

        let mut dir: Option<DIR> = None;

        for (i, pair) in report.windows(2).enumerate() {
            if !Day2::is_valid_pair(&mut dir, pair[0], pair[1]) {
                return (false, i);
            }
        }

        (true, 0)
    }

    fn remove_index<T: Copy>(slice: &[T], index: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(slice.len() - 1);
        result.extend_from_slice(&slice[..index]);
        result.extend_from_slice(&slice[index + 1..]);
        result
    }

    fn is_safe_part2(r: &[i64]) -> bool {
        let (valid, i) = Day2::is_valid_report(r);
        if valid {
            return true;
        }

        if i == 1 && Day2::is_valid_report(&r[1..]).0 {
            return true;
        }

        if Day2::is_valid_report(&Day2::remove_index(r, i)).0 {
            return true;
        }

        if i + 1 < r.len() && Day2::is_valid_report(&Day2::remove_index(r, i + 1)).0 {
            return true;
        }

        false
    }
}

impl Solution for Day2 {
    type Item = Report;

    fn parse_input<P>(path: P) -> io::Result<impl Iterator<Item = Self::Item>>
    where
        P: AsRef<Path>,
    {
        let data = fs::read_to_string(path)?;

        let reports: Vec<Report> = data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect()
            })
            .collect();

        Ok(reports.into_iter())
    }

    fn part1() -> u64 {
        let reports: Vec<Report> = Day2::parse_input("./input/day2").unwrap().collect();
        reports.iter().filter(|&r| Day2::is_safe_part1(r)).count() as u64
    }

    fn part2() -> u64 {
        let reports: Vec<Report> = Day2::parse_input("./input/day2").unwrap().collect();
        reports.iter().filter(|&r| Day2::is_safe_part2(r)).count() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day2_is_safe() {
        let report: Report = vec![7, 6, 4, 2, 1];
        assert!(Day2::is_safe_part1(&report));
    }

    #[test]
    fn test_day2_is_not_safe() {
        let report: Report = vec![1, 2, 7, 8, 9];
        assert!(!Day2::is_safe_part1(&report));
    }

    #[test]
    fn test_day2_part1_example() {
        let reports: Vec<Report> = Day2::parse_input("./example/day2").unwrap().collect();
        let count: usize = reports
            .iter()
            .map(|r| Day2::is_safe_part1(r))
            .filter(|&v| v)
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_day2_is_safe_part2() {
        let r: Report = vec![24, 20, 25, 26];
        assert!(Day2::is_safe_part2(&r));
    }

    #[test]
    fn test_day2_is_not_safe_part2() {
        let r: Report = vec![65, 68, 71, 71, 72, 79];
        assert!(!Day2::is_safe_part2(&r));
    }

    #[test]
    fn test_day2_part2_example() {
        let reports: Vec<Report> = Day2::parse_input("./example/day2").unwrap().collect();
        let count = reports.iter().filter(|&r| Day2::is_safe_part2(r)).count() as u64;
        assert_eq!(count, 4);
    }
}
