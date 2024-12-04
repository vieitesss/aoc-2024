pub enum Dir {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

pub const DIRS: [Dir; 8] = [
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
    pub fn values(&self) -> (isize, isize) {
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

    pub fn from(&self, pos: (isize, isize)) -> (isize, isize) {
        let vals = self.values();
        (pos.0 as isize + vals.0, pos.1 as isize + vals.1)
    }
}

