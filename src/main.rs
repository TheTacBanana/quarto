#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use game::{Game, GameError, GameState};
use player::CliPlayer;

use crate::position::Position;

pub mod board;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;

fn main() -> Result<(), GameError> {
    let mut game = Game::new(
        CliPlayer::new("Eris".to_string()),
        CliPlayer::new("Zoe".to_string()),
    );

    pollster::block_on(game.connect())?;

    loop {
        match pollster::block_on(game.next_turn())? {
            GameState::Win(p) => {
                println!("Game won by {}", p);
                break;
            }
            GameState::Draw => {
                println!("Game is a draw");
                break;
            }
            GameState::Continue => (),
        }
    }

    pollster::block_on(game.disconnect())?;

    Ok(())
}
