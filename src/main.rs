#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

use std::io;

use game::{Game, QuartoError};

use crate::place::Position;

pub mod feat;
pub mod game;
pub mod piece;
pub mod place;

fn main() -> Result<(), QuartoError>{
    let mut game = Game::new(2);

    loop {
        println!("Player {} nominate:", game.current_nominator());
        println!("Remaining Pieces: {:?}", game.remaining_pieces().len());

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap();
        game.nominate_piece(input);

        println!("Player {} place:", game.current_player());

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let split = input.split(",").collect::<Vec<_>>();
        let row: usize = split[0].trim().parse().unwrap();
        let col: usize = split[1].trim().parse().unwrap();
        game.place(Position::from_coord(row, col).unwrap())?;

        game.detect_win();
    }
}