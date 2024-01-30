use async_trait::async_trait;
use rand::Rng;

use crate::{board::Board, player::QuartoPlayer, position::Position};



pub struct MinimaxPlayer {

}

impl MinimaxPlayer {

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
       todo!()
    }

    async fn place(&mut self, board: &Board) -> Position {
        todo!()
    }

    async fn disconnect(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
