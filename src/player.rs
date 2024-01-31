use crate::{board::Board, position::Position};
use rand::prelude::*;

use async_std::io;
use async_trait::async_trait;

#[async_trait]
pub trait QuartoPlayer: 'static + Send + Sync {
    async fn connect(&mut self) -> Result<(), ()>;
    async fn identifier(&mut self) -> &str;
    async fn nominate(&mut self, board: &Board) -> usize;
    async fn place(&mut self, board: &Board) -> Position;
    async fn disconnect(&mut self) -> Result<(), ()>;
}

pub struct CliPlayer {
    name: String,
}

impl CliPlayer {
    pub fn new(name: String) -> Self {
        CliPlayer { name }
    }
}

#[async_trait]
impl QuartoPlayer for CliPlayer {
    async fn connect(&mut self) -> Result<(), ()> {
        Ok(())
    }

    async fn identifier(&mut self) -> &str {
        self.name.as_str()
    }
    async fn nominate(&mut self, board: &Board) -> usize {
        println!("{} nominate:", self.name);
        println!("Pieces {:#16b}", board.piece_bits());

        let mut input = String::new();
        io::stdin().read_line(&mut input).await.unwrap();
        let input: usize = input.trim().parse().unwrap();
        input
    }

    async fn place(&mut self, board: &Board) -> Position {
        println!("{} place {:?}:", self.name, board.nominated_piece());
        println!("{:?}", board.board_bits());

        let mut input = String::new();
        io::stdin().read_line(&mut input).await.unwrap();
        let split = input.split(",").collect::<Vec<_>>();
        let row: usize = split[0].trim().parse().unwrap();
        let col: usize = split[1].trim().parse().unwrap();
        Position::from_coord(row, col).unwrap()
    }

    async fn disconnect(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

pub struct RandomPlayer;

#[async_trait]
impl QuartoPlayer for RandomPlayer {
    async fn connect(&mut self) -> Result<(), ()> {
        Ok(())
    }

    async fn identifier(&mut self) -> &str {
        "RandomPlayer"
    }

    async fn nominate(&mut self, board: &Board) -> usize {
        let pieces = board.piece_indexes();
        let mut rng = rand::thread_rng();
        let piece_index: usize = rng.gen::<u32>() as usize % pieces.len();

        *pieces.get(piece_index).unwrap()
    }

    async fn place(&mut self, board: &Board) -> Position {
        let spaces = board.free_spaces();
        let mut rng = rand::thread_rng();
        let piece_index: usize = rng.gen::<u32>() as usize % spaces.len();

        Position::from_index(*spaces.get(piece_index).unwrap()).unwrap()
    }

    async fn disconnect(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
