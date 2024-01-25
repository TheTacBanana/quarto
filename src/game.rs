use crate::{
    piece::Piece,
    place::{Nominated, Placed, Position},
};

#[derive(Debug, Default)]
pub struct Game {
    remaining_pieces: Vec<Piece>,
    cur_board: [Option<usize>; 16],
    moves: Vec<Placed>,
    next_player: usize,
    player_count: usize,
    nominated: Option<Nominated>,
}

impl Game {
    pub fn new(player_count: usize) -> Self {
        Game {
            remaining_pieces: Piece::all_pieces(),
            cur_board: [None; 16],
            moves: Vec::new(),
            next_player: 0,
            player_count,
            nominated: None,
        }
    }

    pub fn nominated(&self) -> Option<&Nominated> {
        self.nominated.as_ref()
    }

    pub fn nominate_piece(&mut self, nom: Nominated) -> &mut Nominated {
        self.nominated.insert(nom)
    }

    pub fn play(&mut self, pos: Position) -> Result<usize, QuartoError> {
        let placed = Placed::from_nominated(self.nominated.ok_or(QuartoError::NoneNominated)?, pos);
        let index = pos.to_index();
        let next_turn = self.moves.len();

        let square = self.cur_board.get_mut(index).unwrap();
        match square {
            Some(_) => Err(QuartoError::OccupiedSquare)?,
            None => {
                square.insert(next_turn);
                self.moves.push(placed);
            },
        };

        self.next_player += 1;
        Ok(self.next_player)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuartoError {
    NoneNominated,
    OccupiedSquare,
}
