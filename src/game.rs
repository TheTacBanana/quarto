use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    piece::Piece,
    place::{Placed, Position},
};

#[derive(Debug, Default)]
pub struct Game {
    remaining_pieces: Vec<Piece>,
    cur_board: [Option<usize>; 16],
    moves: Vec<Placed>,
    next: (usize, usize), // Placer, Nominator
    player_count: usize,
    nominated: Option<usize>,
}

impl Game {
    pub fn new(player_count: usize) -> Self {
        Game {
            remaining_pieces: Piece::all_pieces(),
            cur_board: [None; 16],
            moves: Vec::new(),
            next: (0, player_count - 1),
            player_count,
            nominated: None,
        }
    }

    #[inline]
    pub fn nominated(&self) -> Option<Piece> {
        self.nominated
            .as_ref()
            .map(|x| *self.remaining_pieces.get(*x).unwrap())
    }

    #[inline]
    pub fn nominate_piece(&mut self, nom: usize) -> Option<Piece> {
        let _ = self.nominated.insert(nom);
        self.nominated()
    }

    pub fn place(&mut self, pos: Position) -> Result<(), QuartoError> {
        let nominated = self.nominated.ok_or(QuartoError::NoneNominated)?;
        let to_play = self.remaining_pieces.remove(nominated);

        let placed = Placed::from_nominated(to_play, pos);
        let index = pos.to_index();
        let next_turn = self.moves.len();

        let square = self.cur_board.get_mut(index).unwrap();
        match square {
            Some(_) => Err(QuartoError::OccupiedSquare)?,
            None => {
                let _ = square.insert(next_turn);
                self.moves.push(placed);
            }
        };

        self.next = ((self.next.0 + 1) % self.player_count, self.next.0);
        Ok(())
    }

    #[inline]
    pub fn current_nominator(&self) -> usize {
        self.next.1
    }

    #[inline]
    pub fn current_player(&self) -> usize {
        self.next.0
    }

    pub fn remaining_pieces(&self) -> &Vec<Piece> {
        &self.remaining_pieces
    }

    pub fn check_four(&self, four: [usize; 4]) -> bool {
        four.iter()
            .map(|&x| self.cur_board.get(x).unwrap())
            .try_fold(u8::MAX, |x, new| {
                new.map(|y| {
                    let piece = self.moves.get(y).unwrap();
                    x & piece.0 .0
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
        ]
        .par_iter()
        .any(|&xs| self.check_four(xs))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuartoError {
    NoneNominated,
    OccupiedSquare,
}

pub mod tests {
    use crate::*;

    pub fn play_piece(game: &mut Game, n: usize, r: usize, c: usize) {
        game.nominate_piece(n);
        game.place(Position::from_coord(r, c).unwrap()).unwrap();
    }

    #[test]
    pub fn row() {
        let mut game = Game::new(2);

        play_piece(&mut game, 0, 1, 0);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 1, 1, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 2, 1, 2);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 3, 1, 3);
        assert!(game.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn col() {
        let mut game = Game::new(2);

        play_piece(&mut game, 0, 0, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 1, 1, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 2, 2, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 3, 3, 1);
        assert!(game.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn back_diag() {
        let mut game = Game::new(2);

        play_piece(&mut game, 0, 0, 0);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 1, 1, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 2, 2, 2);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 3, 3, 3);
        assert!(game.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn forward_diag() {
        let mut game = Game::new(2);

        play_piece(&mut game, 0, 0, 3);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 1, 1, 2);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 2, 2, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 3, 3, 0);
        assert!(game.detect_win() == true, "Didnt detect");
    }
}
