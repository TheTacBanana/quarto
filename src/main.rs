#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use crate::{game::Game, player::RandomPlayer, runner::GameRunner};

pub mod board;
pub mod game;
pub mod piece;
pub mod player;
pub mod position;
pub mod runner;
pub mod minmax;

fn main() {
    let result = pollster::block_on(
        GameRunner::new(u16::MAX as usize, || Game::new(RandomPlayer, RandomPlayer)).run(),
    );
    println!("{:?}", result);
}
