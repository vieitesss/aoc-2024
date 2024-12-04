use super::Solution;
use regex::Regex;
use std::{fs, io, path::Path};

#[derive(Default)]
pub struct Day3 {}

impl Day3 {
    fn sum_muls(data: &str) -> i32 {
        let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        regex
            .captures_iter(&data)
            .map(|c| {
                let x = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

                x * y
            })
            .sum()
    }

    fn remove_disabled(data: &str) -> String {
        let donts: Vec<usize> = data.match_indices("don't()").map(|(i, _)| i).collect();
        let dos: Vec<usize> = data.match_indices("do()").map(|(i, _)| i).collect();

        let donts_len = donts.len();
        let dos_len = dos.len();

        let mut enabled = true;
        let mut from = 0;
        let mut current_do = 0;
        let mut current_dont = 0;

        let mut result = String::from("");

        for i in (0..data.len()).into_iter() {
            if dos[current_do] == i {
                if !enabled {
                    from = i;
                    enabled = true;
                }
                current_do += 1;
            } else if donts[current_dont] == i {
                if enabled {
                    result += &data[from..i];
                    enabled = false;
                }
                current_dont += 1;
            }

            if current_do == dos_len {
                result += &data[from..donts[current_dont]];
                break;
            }

            if current_dont == donts_len {
                result += &data[dos[current_do]..];
                break;
            }
        }

        result
    }
}

impl Solution for Day3 {
    type Item = String;

    fn parse_input<P>(path: P) -> io::Result<impl Iterator<Item = Self::Item>>
    where
        P: AsRef<Path>,
    {
        let data = fs::read_to_string(path);
        Ok(data.into_iter())
    }

    fn part1() -> u64 {
        let data: String = Day3::parse_input("./input/day3").unwrap().collect();
        Day3::sum_muls(&data) as u64
    }

    fn part2() -> u64 {
        let data: String = Day3::parse_input("./input/day3").unwrap().collect();
        let parsed = Day3::remove_disabled(&data);
        Day3::sum_muls(&parsed) as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_part1_count() {
        let data: String = Day3::parse_input("./example/day3").unwrap().collect();
        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        assert_eq!(regex.find_iter(&data).count(), 4);
    }

    #[test]
    fn day3_part1_example() {
        let data: String = Day3::parse_input("./example/day3").unwrap().collect();
        assert_eq!(Day3::sum_muls(&data), 161);
    }

    #[test]
    fn day3_part2_count() {
        let data: String = Day3::parse_input("./example/day3").unwrap().collect();
        assert_eq!(data.match_indices("do()").count(), 1);
        assert_eq!(data.match_indices("don't()").count(), 1);
    }

    #[test]
    fn day3_part2_remove() {
        let data: String = Day3::parse_input("./example/day3").unwrap().collect();
        assert_eq!(
            Day3::remove_disabled(&data),
            String::from("xmul(2,4)&mul[3,7]!^do()?mul(8,5))\n")
        );
    }

    #[test]
    fn day3_part2_remove2() {
        let data = String::from("xdo()mul(2,4)&muldo()[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?don't()mul(8,5))");
        assert_eq!(
            Day3::remove_disabled(&data),
            String::from("xdo()mul(2,4)&muldo()[3,7]!^do()?")
        );
    }

    #[test]
    fn day3_part2_example() {
        let data = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let parsed = Day3::remove_disabled(&data);

        assert_eq!(Day3::sum_muls(&parsed), 48);
    }
}
