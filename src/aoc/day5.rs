use super::Solution;
use std::{collections::HashMap, fs};

#[derive(Default)]
struct Rules(HashMap<usize, Vec<usize>>);

impl Rules {
    fn insert(&mut self, rule: &str) {
        let numbers: Vec<usize> = rule
            .split("|")
            .map(|n| n.parse().expect("A valid number"))
            .collect();
        let key = numbers[0];
        if let Some(sucesors) = self.0.get_mut(&key) {
            sucesors.push(numbers[1]);
        } else {
            self.0.insert(key, vec![numbers[1]]);
        }
    }
}

#[derive(Default)]
pub struct Day5 {
    rules: Rules,
    updates: Vec<Vec<usize>>,
}

impl Day5 {
    fn parse(&mut self, data: &str) {
        let mut rules = Rules::default();
        let mut updates = Vec::new();

        for line in data.lines() {
            if line.contains("|") {
                rules.insert(line);
            } else if line.contains(",") {
                updates.push(
                    line.split(",")
                        .map(|n| n.parse().expect("A valid number"))
                        .collect(),
                );
            }
        }

        self.rules = rules;
        self.updates = updates;
    }

    fn is_valid_update(&self, update: &[usize]) -> bool {
        let update_len = update.len();
        for (i, n) in update.iter().enumerate() {
            if i == update_len - 1 {
                break;
            }
            if let Some(v) = self.rules.0.get(&n) {
                for sucesor in update[i + 1..].iter() {
                    if !v.contains(sucesor) {
                        return false;
                    }
                }
            } else {
                return i == update_len - 1;
            }
        }

        true
    }

    fn ordered(&self, update: &[usize]) -> Vec<usize> {
        assert!(!update.is_empty());
        let mut ordered = vec![update[0]];

        for n in update.iter().skip(1) {
            let mut inserted = false;
            for i in 0..ordered.len() {
                let o = ordered[i];
                if let Some(v) = self.rules.0.get(&o) {
                    if !v.contains(n) {
                        ordered.insert(i, *n);
                        inserted = true;
                        break;
                    }
                } else {
                    ordered.insert(i, *n);
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                ordered.push(*n);
            }
        }

        ordered
    }
}

impl Solution for Day5 {
    fn parse_input(&mut self) {
        self.parse(&fs::read_to_string("./input/day5").unwrap());
    }

    fn part1(&mut self) -> u64 {
        self.updates
            .iter()
            .filter(|u| self.is_valid_update(&u))
            .map(|u| u[u.len() / 2])
            .sum::<usize>() as u64
    }

    fn part2(&mut self) -> u64 {
        self.updates
            .iter()
            .filter(|u| !self.is_valid_update(&u))
            .map(|u| self.ordered(&u))
            .map(|u| u[u.len() / 2])
            .sum::<usize>() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day5_part1_data() {
        let mut day = Day5::default();
        day.parse(
            "
47|53
97|13
97|61

75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );
        assert_eq!(day.rules.0.len(), 2); // Dos números diferentes en el lado izquierdo
        assert_eq!(day.updates.len(), 3);
        assert_eq!(day.rules.0.get(&97), Some(vec![13, 61]).as_ref());
        assert_eq!(day.rules.0.get(&47), Some(vec![53]).as_ref());
        assert_eq!(day.rules.0.get(&53), None);
        assert_eq!(day.updates[1], vec![61, 13, 29]);
    }

    #[test]
    fn day5_part1_valid_update() {
        let mut day = Day5::default();
        day.parse(&fs::read_to_string("./example/day5").unwrap());
        let r: Vec<bool> = day
            .updates
            .iter()
            .map(|u| day.is_valid_update(&u))
            .collect();
        assert_eq!(r, vec![true, true, true, false, false, false]);
    }

    #[test]
    fn day5_part1_example() {
        let mut day = Day5::default();
        day.parse(&fs::read_to_string("./example/day5").unwrap());
        let sum: usize = day
            .updates
            .iter()
            .filter(|u| day.is_valid_update(&u))
            .map(|u| u[u.len() / 2])
            .sum();

        assert_eq!(sum, 143);
    }

    #[test]
    fn day5_part2_ordered() {
        let mut day = Day5::default();
        day.parse(&fs::read_to_string("./example/day5").unwrap());
        assert_eq!(day.ordered(&day.updates[3]), vec![97, 75, 47, 61, 53]);
        assert_eq!(day.ordered(&day.updates[4]), vec![61, 29, 13]);
        assert_eq!(day.ordered(&day.updates[5]), vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn day5_part2_example() {
        let mut day = Day5::default();
        day.parse(&fs::read_to_string("./example/day5").unwrap());
        let sum: usize = day
            .updates
            .iter()
            .filter(|u| !day.is_valid_update(&u))
            .map(|u| day.ordered(&u))
            .map(|u| u[u.len() / 2])
            .sum();

        assert_eq!(sum, 123);
    }
}
