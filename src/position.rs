// 0  1  2  3
// 4  5  6  7
// 8  9  10 11
// 12 13 14 15
#[derive(Debug, Clone, Copy)]
pub struct Position(usize);

impl Position {
    pub fn from_coord(row : impl Into<usize>, col : impl Into<usize>) -> Option<Self> {
        match (row.into(),col.into()) {
            (r,c) if (0..4).contains(&r) && (0..4).contains(&c) => {
                Some(Position(r * 4 + c))
            }
            _ => {
                None
            }
        }
    }

    pub fn from_index(index : impl Into<usize>) -> Option<Self> {
        match index.into() {
            i if (0..16).contains(&i) => {
                Some(Position(i))
            }
            _ => {
                None
            }
        }
    }

    #[inline]
    pub fn to_index(&self) -> usize {
        self.0
    }
}
