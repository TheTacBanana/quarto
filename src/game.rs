use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    board::Board, piece::Piece, player::QuartoPlayer, position::Position
};

pub struct Game {
    next: (usize, usize), // Placer, Nominator
    players: Vec<Box<dyn QuartoPlayer>>,
    current_board: Board,
}

impl Game {
    pub fn new(players : Vec<Box<dyn QuartoPlayer>>) -> Self {
        Game {
            next: (0, players.len() - 1),
            players,
            current_board: Board::new(),
        }
    }

    pub fn next() {

    }
}


pub mod tests {
    use crate::*;

    use self::board::Board;

    pub fn play_piece(board: &mut Board, n: usize, r: usize, c: usize) {
        board.nominate_inplace(n);
        board.place_inplace(Position::from_coord(r, c).unwrap()).unwrap();
    }

    #[test]
    pub fn row() {
        let mut board = Board::new();

        play_piece(&mut board, 0, 1, 0);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 1, 1, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 2, 1, 2);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 3, 1, 3);
        assert!(board.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn col() {
        let mut board = Board::new();

        play_piece(&mut board, 0, 0, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 1, 1, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 2, 2, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 3, 3, 1);
        assert!(board.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn back_diag() {
        let mut game = Board::new();

        play_piece(&mut game, 0, 0, 0);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 1, 1, 1);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 2, 2, 2);
        assert!(game.detect_win() == false, "Invalid");

        play_piece(&mut game, 3, 3, 3);
        assert!(game.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn forward_diag() {
        let mut board = Board::new();

        play_piece(&mut board, 0, 0, 3);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 1, 1, 2);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 2, 2, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 3, 3, 0);
        assert!(board.detect_win() == true, "Didnt detect");
    }

    #[test]
    pub fn quad() {
        let mut board = Board::new();

        play_piece(&mut board, 0, 1, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 1, 1, 2);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 2, 2, 1);
        assert!(board.detect_win() == false, "Invalid");

        play_piece(&mut board, 3, 2, 2);
        assert!(board.detect_win() == true, "Didnt detect");
    }
}
