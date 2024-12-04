use super::matrix::Matrix;

pub fn to_chars_matrix(data: &str) -> Matrix<char> {
    data.to_string()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
