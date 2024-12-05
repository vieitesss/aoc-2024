use super::Solution;
use std::{collections::HashMap, fs, path::Path};

type Column = Vec<i64>;
type Matrix = Vec<Column>;
type Counter = HashMap<u64, u64>;

#[derive(Default)]
pub struct Day1 {
    columns: Matrix,
}

impl Day1 {
    fn get_columns(&mut self, pairs: Vec<(i64, i64)>) {
        let mut columns: Matrix = vec![vec![]; 2];

        for (n1, n2) in pairs {
            columns[0].push(n1);
            columns[1].push(n2);
        }

        columns[0].sort_unstable();
        columns[1].sort_unstable();

        self.columns = columns;
    }

    fn get_distance(&self) -> i64 {
        self.columns[0]
            .iter()
            .zip(&self.columns[1])
            .map(|(&a, &b)| (a - b).abs())
            .sum()
    }

    fn count_numbers(numbers: &[i64]) -> Counter {
        let mut count = HashMap::new();

        for &n in numbers {
            *count.entry(n as u64).or_insert(0) += 1;
        }

        count
    }

    fn get_similarity(&self) -> u64 {
        let count1 = Day1::count_numbers(&self.columns[0]);
        let count2 = Day1::count_numbers(&self.columns[1]);

        count1.iter().fold(0, |total, (&n, &times1)| {
            if let Some(&times2) = count2.get(&n) {
                total + n * times1 * times2
            } else {
                total
            }
        })
    }

    fn parse<P: AsRef<Path>>(&mut self, path: P) {
        let data = fs::read_to_string(path).unwrap();
        let pairs: Vec<(i64, i64)> = data
            .lines()
            .map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok());

                let n1 = nums.next().expect("Expected first number");
                let n2 = nums.next().expect("Expected second number");

                (n1, n2)
            })
            .collect();

        self.get_columns(pairs);
    }
}

impl Solution for Day1 {
    fn parse_input(&mut self) {
        self.parse("./input/day1");
    }

    fn part1(&mut self) -> u64 {
        self.get_distance() as u64
    }

    fn part2(&mut self) -> u64 {
        self.get_similarity()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let mut day = Day1::default();
        day.parse("./example/day1");
        assert_eq!(day.get_distance(), 11);
    }

    #[test]
    fn test_day1_part2_count() {
        let mut day = Day1::default();
        day.parse("./example/day1");

        let matrix1 = Day1::count_numbers(&day.columns[0]);
        match matrix1.get(&3) {
            Some(count) => assert_eq!(*count, 3),
            None => assert!(false),
        }

        let matrix1 = Day1::count_numbers(&day.columns[1]);
        match matrix1.get(&9) {
            Some(count) => assert_eq!(*count, 1),
            None => assert!(false),
        }
    }

    #[test]
    fn test_day1_part2() {
        let mut day = Day1::default();
        day.parse("./example/day1");
        assert_eq!(day.get_similarity(), 31);
    }
}
