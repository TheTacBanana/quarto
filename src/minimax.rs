use async_trait::async_trait;
use ordered_float::OrderedFloat;

use crate::{board::Board, player::QuartoPlayer, position::Position};

pub struct MinimaxPlayer;

impl MinimaxPlayer {
    pub const MAX_DEPTH: usize = 5;

    pub fn maxi_place(&self, depth: usize, board: &Board) -> f32 {
        if depth == 0 {
            return self.evaluate_board(board);
        }
        board
            .free_spaces()
            .iter()
            .map(|i| board.place(Position::from_index(*i).unwrap()).unwrap())
            .map(|b| OrderedFloat(self.maxi_nominate(depth - 1, &b)))
            .max()
            .unwrap()
            .0
    }

    pub fn maxi_nominate(&self, depth: usize, board: &Board) -> f32 {
        if depth == 0 {
            return self.evaluate_board(board);
        }
        board
            .piece_indexes()
            .iter()
            .map(|i| board.nominate(*i).unwrap())
            .map(|b| OrderedFloat(self.mini_place(depth - 1, &b)))
            .min()
            .unwrap()
            .0
    }

    pub fn mini_place(&self, depth: usize, board: &Board) -> f32 {
        if depth == 0 {
            return -self.evaluate_board(board);
        }
        board
            .free_spaces()
            .iter()
            .map(|i| board.place(Position::from_index(*i).unwrap()).unwrap())
            .map(|b| OrderedFloat(self.mini_nominate(depth - 1, &b)))
            .min()
            .unwrap()
            .0

    }

    pub fn mini_nominate(&self, depth: usize, board: &Board) -> f32 {
        if depth == 0 {
            return -self.evaluate_board(board);
        }
        board
            .piece_indexes()
            .iter()
            .map(|i| {
                board.nominate(*i).unwrap()
            })
            .map(|b| OrderedFloat(self.maxi_place(depth - 1, &b)))
            .min()
            .unwrap()
            .0
    }

    pub fn nominate(&mut self, board: &Board) -> usize {
        board
            .piece_indexes()
            .drain(..)
            .map(|piece| {
                let board = board.nominate(piece).unwrap();
                (piece, OrderedFloat(self.mini_place(Self::MAX_DEPTH, &board)))
            })
            .max_by_key(|x| x.1)
            .inspect(|x| println!("Nominate {:?}", x) )
            .unwrap()
            .0
    }

    pub fn place(&mut self, board: &Board) -> Position {
        board
            .free_spaces()
            .drain(..)
            .map(|space| {
                let pos = Position::from_index(space).unwrap();
                let board = board.place(pos).unwrap();
                (pos, OrderedFloat(self.maxi_nominate(Self::MAX_DEPTH, &board)))
            })
            .max_by_key(|x| x.1)
            .inspect(|x| println!("Place {:?}", x) )
            .unwrap()
            .0
    }

    pub fn evaluate_board(&self, board: &Board) -> f32 {
        if board.detect_win() {
            return f32::INFINITY;
        }

        let count = Board::quartos()
            .iter()
            .enumerate()
            .map(|(i, xs)| {
                let xs = xs.iter().filter_map(|&x| board.get_square_index(x));
                let n = xs.count();
                (i, n)
            })
            .max_by(|l, r| l.1.cmp(&r.1))
            .unwrap();
        count.1 as f32
    }
}

#[async_trait]
impl QuartoPlayer for MinimaxPlayer {
    async fn connect(&mut self) -> Result<(), ()> {
        Ok(())
    }

    async fn identifier(&mut self) -> &str {
        "MinMax"
    }

    async fn nominate(&mut self, board: &Board) -> usize {
        self.nominate(board)
    }

    async fn place(&mut self, board: &Board) -> Position {
        self.place(board)
    }

    async fn disconnect(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
