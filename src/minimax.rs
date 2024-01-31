use async_trait::async_trait;
use ordered_float::OrderedFloat;
use rand::Rng;

use crate::{board::Board, player::QuartoPlayer, position::Position};

pub struct MinimaxPlayer;

impl MinimaxPlayer {
    pub const MAX_DEPTH: usize = 5;

    pub fn maxi_place(&self, depth: usize, board: &Board) -> f32 {
        if depth == 0 || board.detect_win(){
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
        if depth == 0 || board.detect_win(){
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
        if depth == 0 || board.detect_win(){
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
        if depth == 0 || board.detect_win(){
            return -self.evaluate_board(board);
        }
        board
            .piece_indexes()
            .iter()
            .map(|i| board.nominate(*i).unwrap())
            .map(|b| OrderedFloat(self.maxi_place(depth - 1, &b)))
            .min()
            .unwrap()
            .0
    }

    pub fn nominate(&mut self, board: &Board) -> usize {
        let moves = board
            .piece_indexes()
            .drain(..)
            .map(|piece| {
                let board = board.nominate(piece).unwrap();
                (
                    piece,
                    OrderedFloat(self.maxi_nominate(Self::MAX_DEPTH, &board)),
                )
            }).collect::<Vec<_>>();

        println!("{:?}", moves);

        let max = moves.iter().max_by_key(|x| x.1).unwrap();
        let moves = moves.iter().filter(|x| x.1 == max.1).collect::<Vec<_>>();

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<u32>() as usize % moves.len();
        let m = moves.get(index).unwrap();
        println!("Nominate {:?}", m);
        m.0
    }

    pub fn place(&mut self, board: &Board) -> Position {
        let moves = board
            .free_spaces()
            .drain(..)
            .map(|space| {
                let pos = Position::from_index(space).unwrap();
                let board = board.place(pos).unwrap();
                (
                    pos,
                    OrderedFloat(self.maxi_nominate(Self::MAX_DEPTH, &board)),
                )
            }).collect::<Vec<_>>();

        println!("{:?}", moves);

        let max = moves.iter().max_by_key(|x| x.1).unwrap();
        let moves = moves.iter().filter(|x| x.1 == max.1).collect::<Vec<_>>();

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<u32>() as usize % moves.len();
        let m = moves.get(index).unwrap();
        println!("Nominate {:?}", m);
        m.0
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
                let common = xs.fold(u8::MAX, |x, y| x & y.0);
                let score = common.count_ones() as f32;
                (i, score)
            })
            .max_by(|l, r| {
                l.1.total_cmp(&r.1)
            })
            .unwrap();

        (16 - board.placed_count()) as f32 + count.1
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
