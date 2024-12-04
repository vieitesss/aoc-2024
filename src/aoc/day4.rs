use super::Solution;
use std::{fs, io, path::Path};

type Row = Vec<char>;
type Matrix = Vec<Row>;

enum Dir {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

const DIRS: [Dir; 8] = [
    Dir::TopLeft,
    Dir::Top,
    Dir::TopRight,
    Dir::Right,
    Dir::BottomRight,
    Dir::Bottom,
    Dir::BottomLeft,
    Dir::Left,
];

impl Dir {
    fn values(&self) -> (isize, isize) {
        match *self {
            Dir::TopLeft => (-1, -1),
            Dir::Top => (-1, 0),
            Dir::TopRight => (-1, 1),
            Dir::Right => (0, 1),
            Dir::BottomRight => (1, 1),
            Dir::Bottom => (1, 0),
            Dir::BottomLeft => (1, -1),
            Dir::Left => (0, -1),
        }
    }

    fn from(&self, pos: (usize, usize)) -> (isize, isize) {
        let vals = self.values();
        (pos.0 as isize + vals.0, pos.1 as isize + vals.1)
    }
}

pub struct Day4 {
    matrix: Matrix,
}

impl Day4 {
    fn new(matrix: Matrix) -> Day4 {
        Day4 { matrix }
    }

    fn to_matrix(data: &str) -> Matrix {
        data.to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn find_x(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, r)| {
                let row_xs: Vec<usize> = r.iter().enumerate().fold(vec![], |mut acc, (i, c)| {
                    if *c == 'X' {
                        acc.push(i);
                    }
                    acc
                });

                row_xs.iter().for_each(|p| acc.push((i, *p)));

                acc
            })
    }

    fn is_outbounds(&self, pos: (isize, isize)) -> bool {
        pos.0 < 0
            || pos.0 as usize >= self.matrix.len()
            || pos.1 < 0
            || pos.1 as usize >= self.matrix[0].len()
    }

    fn find_next(&self, pos: (usize, usize), dir: Dir, find: char) -> bool {
        let checking = dir.from(pos);
        //eprintln!("{:?}, {}", checking, find);
        if self.is_outbounds(checking) {
            return false;
        }

        if self.matrix[checking.0 as usize][checking.1 as usize] == find {
            match find {
                'M' => return self.find_next((checking.0 as usize, checking.1 as usize), dir, 'A'),
                'A' => return self.find_next((checking.0 as usize, checking.1 as usize), dir, 'S'),
                'S' => return true,
                _ => panic!("Should not check for {find}"),
            };
        }

        false
    }

    fn is_xmas(&self, pos: (usize, usize), dir: Dir) -> bool {
        self.find_next(pos, dir, 'M')
    }

    fn count_xmas(&self) -> usize {
        let xs = self.find_x();
        xs.iter().fold(0, |mut acc, &pos| {
            for dir in DIRS {
                if self.is_xmas(pos, dir) {
                    acc += 1;
                }
            }

            acc
        })
    }
}

impl Solution for Day4 {
    type Item = Row;

    fn parse_input<P>(path: P) -> io::Result<impl Iterator<Item = Self::Item>>
    where
        P: AsRef<Path>,
    {
        let data = fs::read_to_string(path).unwrap();

        Ok(Day4::to_matrix(&data).into_iter())
    }

    fn part1() -> u64 {
        let data = Day4::parse_input("./input/day4").unwrap().collect();
        let day = Day4::new(data);
        day.count_xmas() as u64
    }

    fn part2() -> u64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day4_part1_matrix() {
        let data: Matrix = Day4::to_matrix("MMMSXXM\nMSAMXMS");
        assert_eq!(
            data,
            vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S']
            ]
        );
    }

    #[test]
    fn day4_part1_find_x() {
        let data: Matrix = Day4::to_matrix("MMXMSXM\nMMAMSAM\nMSAMXMS");
        let day = Day4::new(data);
        let xs = day.find_x();
        assert_eq!(xs, vec![(0, 2), (0, 5), (2, 4)]);
    }

    #[test]
    fn day4_part1_count_x() {
        let data: Matrix = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        let xs = day.find_x();
        assert_eq!(xs.len(), 19);
    }

    #[test]
    fn day4_part1_dir_from() {
        assert_eq!(Dir::Right.from((0, 5)), (0, 6));
        assert_eq!(Dir::TopLeft.from((0, 5)), (-1, 4));
        assert_eq!(Dir::Top.from((0, 5)), (-1, 5));
        assert_eq!(Dir::TopRight.from((0, 5)), (-1, 6));
        assert_eq!(Dir::BottomRight.from((0, 5)), (1, 6));
        assert_eq!(Dir::BottomLeft.from((0, 5)), (1, 4));
        assert_eq!(Dir::Left.from((0, 5)), (0, 4));
    }

    #[test]
    fn day4_part1_outbounds() {
        let data: Matrix = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert!(day.is_outbounds((2, -1)))
    }

    #[test]
    fn day4_part1_is_xmas() {
        let data: Matrix = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert!(day.is_xmas((0, 5), Dir::Right));
        assert!(day.is_xmas((0, 4), Dir::BottomRight));
    }

    #[test]
    fn day4_part1_is_not_xmas() {
        let data: Matrix = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert!(!day.is_xmas((2, 2), Dir::Left));
        assert!(!day.is_xmas((7, 2), Dir::BottomRight));
    }

    #[test]
    fn day4_part1_example_count() {
        let data: Matrix = Day4::parse_input("./example/day4").unwrap().collect();
        let day = Day4::new(data);
        assert_eq!(day.count_xmas(), 18);
    }
}
