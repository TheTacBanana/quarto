use crate::piece::Piece;

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

    pub fn to_index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Placed(pub Piece, pub Position);

impl Placed {
    pub fn from_nominated(nominated: Piece, pos : Position) -> Self {
        Placed(nominated, pos)
    }
}
