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
    fn is_safe(report: &[i64]) -> bool {
        if report.len() < 2 {
            return false
        }

        let mut dir: Option<DIR> = None;

        for pair in report.windows(2) {
            let (current, next) = (pair[0], pair[1]);
            let diff = next - current;
            let abs_diff = diff.abs();

            if abs_diff == 0 || abs_diff > 3 {
                return false;
            }

            let current_dir = if diff > 0 {
                DIR::DECREASING
            } else {
                DIR::INCREASING
            };

            if let Some(prev_dir) = &dir {
                if prev_dir != &current_dir {
                    return false;
                }
            } else {
                dir = Some(current_dir);
            }
        }

        true
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
        reports
            .iter()
            .map(|r| Day2::is_safe(&r))
            .filter(|&v| v)
            .count() as u64
    }

    fn part2() -> u64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day2_is_safe() {
        // 7 6 4 2 1
        let report: Report = vec![7, 6, 4, 2, 1];
        assert!(Day2::is_safe(&report));
    }

    #[test]
    fn test_day2_is_not_safe() {
        let report: Report = vec![1, 2, 7, 8, 9];
        assert!(!Day2::is_safe(&report));
    }

    #[test]
    fn test_day2_part1_example() {
        let reports: Vec<Report> = Day2::parse_input("./example/day2").unwrap().collect();
        let count: usize = reports.iter().map(|r| Day2::is_safe(r)).filter(|&v| v).count();
        assert_eq!(count, 2);
    }
}
