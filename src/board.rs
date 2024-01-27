use crate::{
    game::QuartoError,
    piece::Piece,
    place::{Placed, Position},
};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    placed: u16,
    board: [u8; 16],
    remaining: u16,
    nominated: Option<usize>,
}

impl Board {
    pub const fn piece_order() -> [u8; 16] {
        [
            0b1010101, 0b1010110, 0b1011001, 0b1011010, 0b1100101, 0b1100110, 0b1101001, 0b1101010,
            0b10010101, 0b10010110, 0b10011001, 0b10011010, 0b10100101, 0b10100110, 0b10101001,
            0b10101010,
        ]
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
    pub fn get_square(&self, pos: Position) -> Option<Piece> {
        let i = pos.to_index();
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

    pub fn play_inplaced(&mut self, position : Position) -> Result<(), QuartoError> {
        let nom = self.nominated.take().ok_or(QuartoError::NoneNominated)?;
        let i = position.to_index();
        if self.placed & 1 << i != 0 {
            Err(QuartoError::OccupiedSquare)?
        }

        self.placed |= 1 << i;
        self.board[i] = Board::piece_order()[nom];
        Ok(())
    }

    pub fn play(&self, position : Position) -> Result<Board, QuartoError> {
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
}
