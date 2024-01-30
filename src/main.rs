#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use runner::GameRunner;

use crate::{game::Game, player::RandomPlayer};

pub mod board;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;
pub mod runner;

fn main() {
    let result = GameRunner::new(u16::MAX as usize, || Game::new(RandomPlayer, RandomPlayer)).run();

    println!("{:?}", result);
}
