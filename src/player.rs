use crate::{board::Board, position::Position};

pub trait QuartoPlayer : 'static {
    fn nominate(&mut self, board : &Board) -> usize;
    fn play(&mut self, board : &Board) -> Position;
}