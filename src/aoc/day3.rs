use super::Solution;
use regex::Regex;
use std::fs;

#[derive(Default)]
pub struct Day3 {
    data: String,
}

impl Day3 {
    fn sum_muls(&self) -> u64 {
        let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        regex
            .captures_iter(&self.data)
            .map(|c| {
                let x = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<u64>().unwrap();

                x * y
            })
            .sum()
    }

    fn remove_disabled(&self) -> String {
        let mut result = String::new();
        let mut enabled = true;
        let mut index = 0;

        while index < self.data.len() {
            let remaining = &self.data[index..];
            if remaining.starts_with("don't()") {
                enabled = false;
                index += 7;
            } else if remaining.starts_with("do()") {
                enabled = true;
                index += 4;
            } else {
                if enabled {
                    let ch = remaining.chars().next().unwrap();
                    result.push(ch);
                }
                index += 1;
            }
        }

        result
    }
}

impl Solution for Day3 {
    fn parse_input(&mut self) {
        let data = fs::read_to_string("./input/day3").unwrap();
        self.data = data;
    }

    fn part1(&mut self) -> u64 {
        self.sum_muls()
    }

    fn part2(&mut self) -> u64 {
        self.data = self.remove_disabled();
        self.sum_muls()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_part1_count() {
        let data: String = fs::read_to_string("./example/day3").unwrap();
        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        assert_eq!(regex.find_iter(&data).count(), 4);
    }

    #[test]
    fn day3_part1_example() {
        let data: String = fs::read_to_string("./example/day3").unwrap();
        let mut day = Day3::default();
        day.data = data;
        assert_eq!(day.sum_muls(), 161);
    }

    #[test]
    fn day3_part2_count() {
        let data: String = fs::read_to_string("./example/day3").unwrap();
        assert_eq!(data.match_indices("do()").count(), 1);
        assert_eq!(data.match_indices("don't()").count(), 1);
    }

    #[test]
    fn day3_part2_remove() {
        let data: String = fs::read_to_string("./example/day3").unwrap();
        let mut day = Day3::default();
        day.data = data;
        assert_eq!(
            day.remove_disabled(),
            String::from("xmul(2,4)&mul[3,7]!^?mul(8,5))\n")
        );
    }

    #[test]
    fn day3_part2_remove2() {
        let data = String::from("xdo()mul(2,4)&muldo()[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?don't()mul(8,5))");
        let mut day = Day3::default();
        day.data = data;
        assert_eq!(day.remove_disabled(), String::from("xmul(2,4)&mul[3,7]!^?"));
    }

    #[test]
    fn day3_part2_example() {
        let data = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let mut day = Day3::default();
        day.data = data;
        day.data = day.remove_disabled();
        assert_eq!(day.sum_muls(), 48);
    }
}
