#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use game::{Game, GameError};
use player::CliPlayer;

use crate::position::Position;

pub mod board;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;

fn main() -> Result<(), GameError> {
    let mut game = Game::new(vec![
        Box::new(CliPlayer::new("Eris".to_string())),
        Box::new(CliPlayer::new("Zoe".to_string())),
    ]);

    pollster::block_on(game.connect());

    loop {
        let turn_result = pollster::block_on(game.next_turn())?;
        if !turn_result {
            println!("Game won by {}", game.placer());
            break;
        }
    }

    pollster::block_on(game.disconnect());

    Ok(())
}
