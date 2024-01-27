#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

use board::QuartoError;
use game::{Game, GameError};

use crate::position::Position;

pub mod feat;
pub mod game;
pub mod piece;
pub mod position;
pub mod player;
pub mod board;

fn main() -> Result<(), GameError>{
    let mut game = Game::new(Vec::new());

    loop {
        pollster::block_on(game.next_turn())?
    }
}