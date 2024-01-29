use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Piece(pub u8);

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}
