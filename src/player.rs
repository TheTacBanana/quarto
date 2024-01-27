use crate::{board::Board, position::Position};

use async_trait::async_trait;

#[async_trait]
pub trait QuartoPlayer : 'static {
    async fn nominate(&mut self, board : &Board) -> usize;
    async fn place(&mut self, board : &Board) -> Position;
}