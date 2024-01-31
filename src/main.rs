#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use crate::{game::Game, minimax::MinimaxPlayer, player::RandomPlayer, runner::GameRunner};

pub mod board;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;
pub mod runner;
pub mod minimax;

fn main() {
    let result = pollster::block_on(
        GameRunner::new(16, || Game::new(MinimaxPlayer, RandomPlayer)).run(),
    );
    println!("{:?}", result);
}
