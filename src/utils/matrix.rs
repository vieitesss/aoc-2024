pub type Row<T> = Vec<T>;
pub type Matrix<T> = Vec<Row<T>>;

pub trait MatrixTrait<T> {
    fn is_outbounds(&self, pos: (isize, isize)) -> bool;
    fn get_pos(&self, pos: (isize, isize)) -> Option<&T>;
    fn find_element(&self, element: &T) -> Vec<(usize, usize)>
    where
        T: PartialEq;
}

impl<T> MatrixTrait<T> for Matrix<T> {
    fn is_outbounds(&self, pos: (isize, isize)) -> bool {
        pos.0 < 0
            || pos.1 < 0
            || pos.0 as usize >= self.len()
            || self.get(0).map_or(true, |row| pos.1 as usize >= row.len())
    }

    fn get_pos(&self, pos: (isize, isize)) -> Option<&T> {
        if self.is_outbounds(pos) {
            return None;
        }

        Some(&self[pos.0 as usize][pos.1 as usize])
    }

    fn find_element(&self, element: &T) -> Vec<(usize, usize)>
    where
        T: PartialEq,
    {
        let mut positions = Vec::new();

        for (i, row) in self.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if item == element {
                    positions.push((i, j));
                }
            }
        }

        positions
    }
}
