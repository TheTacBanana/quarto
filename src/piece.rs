use std::fmt::Debug;

use crate::feat::Feature;

#[derive(Clone, Copy)]
pub struct Piece(pub u8);

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl Piece {
    pub fn from_feat(f1: Feature, f2: Feature, f3: Feature, f4: Feature) -> Self {
        Piece(f1 as u8 | f2 as u8 | f3 as u8 | f4 as u8)
    }

    pub fn all_pieces() -> Vec<Piece> {
        let left = Feature::left();
        let right = Feature::right();

        let mut pieces = Vec::new();

        for i in 0..16 {

            pieces.push(
                Piece::from_feat(
                    {
                        if i & 0b1 == 0 {
                            left[0]
                        } else {
                            right[0]
                        }
                    },
                    {
                        if i & 0b10 == 0 {
                            left[1]
                        } else {
                            right[1]
                        }
                    },
                    {
                        if i & 0b100 == 0 {
                            left[2]
                        } else {
                            right[2]
                        }
                    },
                    {
                        if i & 0b1000 == 0 {
                            left[3]
                        } else {
                            right[3]
                        }
                    },
                )
            );
        }
        pieces
    }
}
