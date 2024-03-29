use std::fmt::Debug;

use crate::{piece::Piece, position::Position};

#[derive(Clone, Copy)]
pub struct Board {
    placed: u16,
    board: [u8; 16],
    remaining_pieces: u16,
    nominated: Option<usize>,
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in 0..4 {
            for r in 0..4 {
                write!(
                    f,
                    "{}",
                    if self.get_square_position(Position::from_coord(r as usize, c as usize).unwrap()).is_some() {
                        "1"
                    } else {
                        "0"
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    /// Order of pieces in the game, each piece is represented as a u8 with a bit for each trait
    pub const fn piece_order() -> [u8; 16] {
        [
            0b1010101, 0b1010110, 0b1011001, 0b1011010, 0b1100101, 0b1100110, 0b1101001, 0b1101010,
            0b10010101, 0b10010110, 0b10011001, 0b10011010, 0b10100101, 0b10100110, 0b10101001,
            0b10101010,
        ]
    }

    pub const fn quartos() -> [[usize; 4]; 19] {
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
            [0, 1, 4, 5],   // Quads
            [1, 2, 5, 6],
            [2, 3, 6, 7],
            [4, 5, 8, 9],
            [5, 6, 9, 10],
            [6, 7, 10, 11],
            [8, 9, 12, 13],
            [9, 10, 13, 14],
            [10, 11, 14, 15],
        ]
    }

    pub fn new() -> Self {
        Board {
            placed: 0,
            board: [0; 16],
            remaining_pieces: u16::MAX,
            nominated: None,
        }
    }

    pub fn board_bits(&self) -> &[u8; 16] {
        &self.board
    }

    #[inline]
    pub fn piece_bits(&self) -> u16 {
        self.remaining_pieces
    }

    /// Iterator over pieces which have not been placed
    #[inline]
    pub fn piece_indexes(&self) -> Vec<usize> {
        let bits = self.remaining_pieces;
        (0..16)
            .into_iter()
            .filter_map(|x| if bits & 1 << x > 0 { Some(x) } else { None })
            .collect()
    }

    #[inline]
    pub fn space_bits(&self) -> u16 {
        self.placed
    }

    pub fn placed_count(&self) -> usize {
        self.placed.count_ones() as usize
    }

    /// Iterator over spaces with placed pieces
    #[inline]
    pub fn taken_spaces(&self) -> Vec<usize> {
        let bits = self.placed;
        (0..16)
            .into_iter()
            .filter_map(|x| if bits & 1 << x > 0 { Some(x) } else { None })
            .collect()
    }

    /// Iterator over free spaces
    #[inline]
    pub fn free_spaces(&self) -> Vec<usize> {
        let bits = !self.placed;
        (0..16)
            .into_iter()
            .filter_map(|x| if bits & 1 << x > 0 { Some(x) } else { None })
            .collect()
    }

    #[inline]
    pub fn nominated_index(&self) -> Option<usize> {
        self.nominated
    }

    #[inline]
    pub fn nominated_piece(&self) -> Option<Piece> {
        self.nominated.map(|x| Piece(Board::piece_order()[x]))
    }

    #[inline]
    pub fn get_square_position(&self, pos: Position) -> Option<Piece> {
        self.get_square_index(pos.to_index())
    }

    #[inline]
    pub fn get_square_index(&self, i: usize) -> Option<Piece> {
        (self.placed & (1 << i) > 0).then(|| Piece(self.board[i]))
    }

    /// Mutates the current board into having a nominated piece
    pub fn nominate_inplace(&mut self, piece: usize) -> Result<(), QuartoError> {
        self.nominated = ((self.remaining_pieces & 1 << piece) != 0)
            .then(|| Some(piece))
            .ok_or(QuartoError::PieceNotAvailable)?;
        Ok(())
    }

    // Creates a new board from self with the nominated piece
    pub fn nominate(&self, piece: usize) -> Result<Board, QuartoError> {
        let mut board = self.clone();
        match board.nominate_inplace(piece) {
            Ok(_) => Ok(board),
            Err(e) => Err(e),
        }
    }

    // Mutates the current board by placing the nominated piece
    pub fn place_inplace(&mut self, position: Position) -> Result<(), QuartoError> {
        let nom = self.nominated.take().ok_or(QuartoError::NoneNominated)?;
        let i = position.to_index();
        if self.placed & 1 << i != 0 {
            Err(QuartoError::OccupiedSquare)?
        }

        self.placed |= 1 << i;
        self.remaining_pieces = self.remaining_pieces & !(1 << nom);
        self.board[i] = Board::piece_order()[nom];
        Ok(())
    }

    // Creates a new board from self by placing the nominated Piece
    pub fn place(&self, position: Position) -> Result<Board, QuartoError> {
        let mut board = self.clone();
        match board.place_inplace(position) {
            Ok(_) => Ok(board),
            Err(e) => Err(e),
        }
    }

    pub fn check_four(&self, four: [usize; 4]) -> bool {
        four.iter()
            .map(|&x| self.get_square_index(x))
            .try_fold(u8::MAX, |x, new| new.map(|y| x & y.0))
            .is_some_and(|x| x > 0)
    }

    pub fn detect_win(&self) -> bool {
        Board::quartos().iter().any(|&xs| self.check_four(xs))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuartoError {
    NoneNominated,
    PieceNotAvailable,
    OccupiedSquare,
}
