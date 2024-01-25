use crate::piece::Piece;

// 0  1  2  3
// 4  5  6  7
// 8  9  10 11
// 12 13 14 15
#[derive(Debug, Clone, Copy)]
pub struct Position(usize);

impl Position {
    pub fn from_coord(row : impl Into<usize>, col : impl Into<usize>) -> Self {
        Position(col.into() * 4 + row.into())
    }

    pub fn to_index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nominated(Piece);

impl Nominated {
    pub fn new(piece: Piece) -> Self {
        Nominated(piece)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Placed(Piece, Position);

impl Placed {
    pub fn from_nominated(nominated: Nominated, pos : Position) -> Self {
        Placed(nominated.0, pos)
    }
}
