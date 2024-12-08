use super::Solution;
use std::{fs, ops};

#[derive(Default)]
pub struct Day7 {
    tests: Vec<usize>,
    operators: Vec<Vec<usize>>,
}

#[derive(PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Con,
}

impl Day7 {
    fn parse(&mut self, data: &str) {
        for line in data.lines() {
            let sides: Vec<&str> = line.split(":").collect();
            self.tests.push(sides[0].parse().expect("A valid number"));
            self.operators.push(
                sides[1]
                    .split_whitespace()
                    .map(|n| n.parse().expect("A valid number"))
                    .collect(),
            );
        }
    }

    fn cals(&self, test: usize, op: Op, pos: usize, res: usize, ops: &[usize]) -> usize {
        if pos == ops.len() {
            return 0;
        }

        let t = if op == Op::Add {
            res + ops[pos]
        } else if op == Op::Mul {
            res * ops[pos]
        } else {
            format!("{}{}", res, ops[pos])
                .parse()
                .expect("A valid number")
        };

        if t == test {
            return 1;
        }

        if t > test {
            return 0;
        }

        return self.cals(test, Op::Mul, pos + 1, t, ops)
            + self.cals(test, Op::Add, pos + 1, t, ops);
    }

    fn cals_part2(&self, test: usize, op: Op, pos: usize, res: usize, ops: &[usize]) -> usize {
        if pos == ops.len() {
            return 0;
        }

        let t = if op == Op::Add {
            res + ops[pos]
        } else if op == Op::Mul {
            res * ops[pos]
        } else {
            format!("{}{}", res, ops[pos])
                .parse()
                .expect("A valid number")
        };

        if t == test {
            return 1;
        }

        if t > test {
            return 0;
        }

        return self.cals_part2(test, Op::Mul, pos + 1, t, ops)
            + self.cals_part2(test, Op::Add, pos + 1, t, ops)
            + self.cals_part2(test, Op::Con, pos + 1, t, ops);
    }

    fn calculate_calibrations(&self, test: usize) -> usize {
        let operators = &self.operators[test];
        let t = self.tests[test];
        self.cals(t, Op::Mul, 1, operators[0], &operators)
            + self.cals(t, Op::Add, 1, operators[0], &operators)
    }

    fn calculate_calibrations_part2(&self, test: usize) -> usize {
        let operators = &self.operators[test];
        let t = self.tests[test];
        self.cals_part2(t, Op::Mul, 1, operators[0], &operators)
            + self.cals_part2(t, Op::Add, 1, operators[0], &operators)
            + self.cals_part2(t, Op::Con, 1, operators[0], &operators)
    }
}

impl Solution for Day7 {
    fn parse_input(&mut self) {
        self.parse(&fs::read_to_string("./input/day7").unwrap());
    }

    fn part1(&mut self) -> u64 {
        self.tests
            .iter()
            .enumerate()
            .filter(|(i, _)| self.calculate_calibrations(*i) > 0)
            .map(|(_, t)| t)
            .sum::<usize>() as u64
    }

    fn part2(&mut self) -> u64 {
        self.tests
            .iter()
            .enumerate()
            .filter(|(i, _)| self.calculate_calibrations_part2(*i) > 0)
            .map(|(_, t)| t)
            .sum::<usize>() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Day7 {
        fn new() -> Day7 {
            let mut day = Day7::default();
            day.parse(&fs::read_to_string("./example/day7").unwrap());

            day
        }
    }

    #[test]
    fn day7_part1_example() {
        let day = Day7::new();
        let total: usize = day
            .tests
            .iter()
            .enumerate()
            .filter(|(i, _)| day.calculate_calibrations(*i) > 0)
            .map(|(_, t)| t)
            .sum();
        assert_eq!(total, 3749);
    }

    #[test]
    fn day7_part2_example() {
        let day = Day7::new();
        let total: usize = day
            .tests
            .iter()
            .enumerate()
            .filter(|(i, _)| day.calculate_calibrations_part2(*i) > 0)
            .map(|(_, t)| t)
            .sum();
        assert_eq!(total, 11387);
    }
}
