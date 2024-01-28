#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

use board::QuartoError;
use game::{Game, GameError};
use player::CliPlayer;

use crate::position::Position;

pub mod board;
pub mod feat;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;

fn main() -> Result<(), GameError> {
    let mut game = Game::new(vec![
        Box::new(CliPlayer::new("Eris".to_string())),
        Box::new(CliPlayer::new("Zoe".to_string())),
    ]);

    loop {
        pollster::block_on(game.next_turn())?
    }
}
