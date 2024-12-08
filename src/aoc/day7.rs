use super::Solution;
use std::fs;

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

    fn cals(
        &self,
        test: usize,
        op: Op,
        pos: usize,
        res: usize,
        ops: &[usize],
        part: usize,
    ) -> bool {
        if pos == ops.len() {
            return false;
        }

        let t = if op == Op::Add {
            res + ops[pos]
        } else if op == Op::Mul {
            res * ops[pos]
        } else {
            // x5 faster than parsing to String and back to usize!!!!
            let power = ((ops[pos] + 1) as f64).log10().ceil() as u32;
            res * (10_usize.pow(power)) as usize + ops[pos]
        };

        if t == test {
            return true;
        }

        if t > test {
            return false;
        }

        if part == 1 {
            self.cals(test, Op::Mul, pos + 1, t, ops, part)
                || self.cals(test, Op::Add, pos + 1, t, ops, part)
        } else {
            self.cals(test, Op::Mul, pos + 1, t, ops, part)
                || self.cals(test, Op::Add, pos + 1, t, ops, part)
                || self.cals(test, Op::Con, pos + 1, t, ops, part)
        }
    }

    fn calculate_calibrations(&self, test: usize, part: usize) -> bool {
        let operators = &self.operators[test];
        let t = self.tests[test];

        if part == 1 {
            self.cals(t, Op::Mul, 1, operators[0], &operators, part)
                || self.cals(t, Op::Add, 1, operators[0], &operators, part)
        } else {
            self.cals(t, Op::Mul, 1, operators[0], &operators, part)
                || self.cals(t, Op::Add, 1, operators[0], &operators, part)
                || self.cals(t, Op::Con, 1, operators[0], &operators, part)
        }
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
            .filter(|(i, _)| self.calculate_calibrations(*i, 1))
            .map(|(_, t)| t)
            .sum::<usize>() as u64
    }

    fn part2(&mut self) -> u64 {
        self.tests
            .iter()
            .enumerate()
            .filter(|(i, _)| self.calculate_calibrations(*i, 2))
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
            .filter(|(i, _)| day.calculate_calibrations(*i, 1))
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
            .filter(|(i, _)| day.calculate_calibrations(*i, 2))
            .map(|(_, t)| t)
            .sum();
        assert_eq!(total, 11387);
    }
}
