use crate::feat::Feature;

#[derive(Debug, Clone, Copy)]
pub struct Piece(u8);

impl Piece {
    pub fn from_feat(f1: Feature, f2: Feature, f3: Feature, f4: Feature) -> Self {
        Piece(f1 as u8 | f2 as u8 | f3 as u8 | f4 as u8)
    }
}
