use crate::{board::Board, position::Position};

use async_trait::async_trait;
use async_std::io;

#[async_trait]
pub trait QuartoPlayer : 'static {
    async fn nominate(&mut self, board : &Board) -> usize;
    async fn place(&mut self, board : &Board) -> Position;
}

pub struct CliPlayer {
    name : String,
}

impl CliPlayer {
    pub fn new(name : String) -> Self {
        CliPlayer { name }
    }
}

#[async_trait]
impl QuartoPlayer for CliPlayer {
    async fn nominate(&mut self, board : &Board) -> usize {
        println!("{:?} nominate:", self.name);
        println!("Pieces {:#16b}", board.piece_bits());

        let mut input = String::new();
        io::stdin().read_line(&mut input).await.unwrap();
        let input: usize = input.trim().parse().unwrap();
        input
    }

    async fn place(&mut self, board : &Board) -> Position {
        println!("{:?} place {:?}:", self.name, board.nominated_piece());
        println!("{:?}", board.board_bits());

        let mut input = String::new();
        io::stdin().read_line(&mut input).await.unwrap();
        let split = input.split(",").collect::<Vec<_>>();
        let row: usize = split[0].trim().parse().unwrap();
        let col: usize = split[1].trim().parse().unwrap();
        Position::from_coord(row, col).unwrap()
    }
}