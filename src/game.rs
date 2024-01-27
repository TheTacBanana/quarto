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

    pub fn detect_win(&self) -> bool {
        let rows = self.cur_board.iter().array_chunks::<4>().any(|xs| {
            xs.iter()
                .try_fold(u8::MAX, |x, new| {
                    new.map(|y| {
                        let piece = self.moves.get(y).unwrap();
                        x & piece.0 .0
                    })
                })
                .is_some_and(|x| x > 0)
        });

        let mut cols = self.cur_board.iter().array_chunks::<4>().take(4);
        let first = cols.next().unwrap();
        let cols = first
            .iter()
            .zip(cols.next().unwrap())
            .zip(cols.next().unwrap())
            .zip(cols.next().unwrap());

        let cols = cols.map(|(((&&a, &b), &c), &d)| (a, b, c, d)).any(|x| {
            if let (Some(a), Some(b), Some(c), Some(d)) = x {
                let a = self.moves.get(a).unwrap();
                let b = self.moves.get(b).unwrap();
                let c = self.moves.get(c).unwrap();
                let d = self.moves.get(d).unwrap();

                u8::MAX & a.0 .0 & b.0 .0 & c.0 .0 & d.0 .0 > 0
            } else {
                false
            }
        });

        let diag = [[0, 5, 10, 15], [3, 6, 9, 12]].iter().any(|xs| {
            xs.iter()
                .map(|&x| {
                    self.cur_board
                        .get(x)
                        .unwrap()
                        .and_then(|x| self.moves.get(x))
                })
                .try_fold(u8::MAX, |x, new| new.map(|y| x & y.0 .0))
                .is_some_and(|x| x > 0)
        });

        rows || cols || diag
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
