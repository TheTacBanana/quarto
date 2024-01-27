use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{piece::Piece, position::Position};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    placed: u16,
    board: [u8; 16],
    remaining: u16,
    nominated: Option<usize>,
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub const fn piece_order() -> [u8; 16] {
        [
            0b1010101, 0b1010110, 0b1011001, 0b1011010, 0b1100101, 0b1100110, 0b1101001, 0b1101010,
            0b10010101, 0b10010110, 0b10011001, 0b10011010, 0b10100101, 0b10100110, 0b10101001,
            0b10101010,
        ]
    }

    pub fn new() -> Self {
        Board {
            placed: 0,
            board: [0; 16],
            remaining: u16::MAX,
            nominated: None,
        }
    }

    #[inline]
    pub fn piece_bits(&self) -> u16 {
        self.remaining
    }

    #[inline]
    pub fn space_bits(&self) -> u16 {
        self.placed
    }

    #[inline]
    pub fn get_square_position(&self, pos: Position) -> Option<Piece> {
        self.get_square_index(pos.to_index())
    }

    #[inline]
    pub fn get_square_index(&self, i: usize) -> Option<Piece> {
        (self.placed & (1 << i) > 0).then(|| Piece(self.board[i]))
    }

    pub fn nominate_inplace(&mut self, piece: usize) -> Result<(), QuartoError> {
        self.nominated = (self.remaining & 1 << piece != 0)
                .then(|| Some(piece))
                .ok_or(QuartoError::PieceNotAvailable)?;
        Ok(())
    }

    pub fn nominate(&self, piece: usize) -> Result<Board, QuartoError> {
        Ok(Board {
            placed: self.placed,
            board: self.board,
            remaining: self.remaining,
            nominated: (self.remaining & 1 << piece != 0)
                .then(|| Some(piece))
                .ok_or(QuartoError::PieceNotAvailable)?,
        })
    }

    pub fn place_inplace(&mut self, position : Position) -> Result<(), QuartoError> {
        let nom = self.nominated.take().ok_or(QuartoError::NoneNominated)?;
        let i = position.to_index();
        if self.placed & 1 << i != 0 {
            Err(QuartoError::OccupiedSquare)?
        }

        self.placed |= 1 << i;
        self.board[i] = Board::piece_order()[nom];
        Ok(())
    }

    pub fn place(&self, position : Position) -> Result<Board, QuartoError> {
        let nom = self.nominated.clone().ok_or(QuartoError::NoneNominated)?;
        let i = position.to_index();
        if self.placed & 1 << i != 0 {
            Err(QuartoError::OccupiedSquare)?
        }

        Ok(Board {
            placed: self.placed | 1 << i,
            board: {
                let mut b = self.board;
                b[i] = Board::piece_order()[nom];
                b
            },
            remaining: self.remaining & !(1 << nom),
            nominated: None,
        })
    }

    pub fn check_four(&self, four: [usize; 4]) -> bool {
        four.iter()
            .map(|&x| self.get_square_index(x))
            .try_fold(u8::MAX, |x, new| {
                new.map(|y| {
                    x & y.0
                })
            })
            .is_some_and(|x| x > 0)
    }

    pub fn detect_win(&self) -> bool {
        [
            [0, 1, 2, 3], // Rows
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
            [0, 4, 8, 12], // Cols
            [1, 5, 9, 13],
            [2, 6, 10, 14],
            [3, 7, 11, 15],
            [0, 5, 10, 15], // Back Diag
            [3, 6, 9, 12],  // Forward Diag

            [0, 1, 4, 5], // Quads
            [1, 2, 5, 6],
            [2, 3, 6, 7],

            [4, 5, 8, 9],
            [5, 6, 9, 10],
            [6, 7, 10, 11],

            [8, 9, 12, 13],
            [9, 10, 13, 14],
            [10, 11, 14, 15]
        ]
        .par_iter()
        .any(|&xs| self.check_four(xs))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuartoError {
    NoneNominated,
    PieceNotAvailable,
    OccupiedSquare,
}