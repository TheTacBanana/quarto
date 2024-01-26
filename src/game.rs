use crate::{piece::Piece, place::{Placed, Position}};

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
        self.nominated.as_ref().map(|x| *self.remaining_pieces.get(*x).unwrap())
    }

    #[inline]
    pub fn nominate_piece(&mut self, nom : usize) -> Option<Piece> {
        let _ = self.nominated.insert(nom);
        self.nominated()
    }

    pub fn play(&mut self, pos: Position) -> Result<(), QuartoError> {
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
            },
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

    pub fn remaining_pieces(&self) -> &Vec<Piece>{
        &self.remaining_pieces
    }

    pub fn detect_win(&self) -> bool {
        println!("{:?}", self.cur_board.iter().array_chunks::<4>());
        let rows = self.cur_board.iter().array_chunks::<4>().any(
            |xs| {
                xs.iter().try_fold(u8::MAX, |x, new| {
                    new.map(|y| {
                        let piece = self.moves.get(y).unwrap();
                        x & piece.0.0
                    })
                }).is_some_and(|x| x > 0)
            }
        );

        println!("{:?}", rows);

        rows
    }

}

#[derive(Debug, Clone, Copy)]
pub enum QuartoError {
    NoneNominated,
    OccupiedSquare,
}

pub mod tests {
    use crate::*;

    #[test]
    pub fn row() {
        let mut game = Game::new(2);

        game.nominate_piece(0);
        game.place(Position::from_coord(1usize, 0usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(1);
        game.place(Position::from_coord(1usize, 1usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(2);
        game.place(Position::from_coord(1usize, 2usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(3);
        game.place(Position::from_coord(1usize, 3usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn col() {
        let mut game = Game::new(2);

        game.nominate_piece(0);
        game.place(Position::from_coord(0usize, 1usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(1);
        game.place(Position::from_coord(1usize, 1usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(2);
        game.place(Position::from_coord(2usize, 1usize).unwrap())
            .unwrap();

        assert!(game.detect_win() == false, "Invalid");

        game.nominate_piece(3);
        game.place(Position::from_coord(3usize, 1usize).unwrap())
            .unwrap();

        println!("{:?}", game.moves);

        assert!(game.detect_win() == true, "Didnt detect");
    }
}
