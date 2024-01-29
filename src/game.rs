use std::time::Duration;

use async_std::future::timeout;

use crate::{
    board::{Board, QuartoError},
    player::QuartoPlayer,
};

pub struct Game {
    next: (usize, usize), // Placer, Nominator
    players: Vec<Box<dyn QuartoPlayer>>,
    connected_players: Vec<usize>,
    board: Board,
}

impl Game {
    pub fn new(players: Vec<Box<dyn QuartoPlayer>>) -> Self {
        Game {
            next: (0, players.len() - 1),
            players,
            connected_players: Vec::new(),
            board: Board::new(),
        }
    }

    #[inline]
    pub fn placer(&self) -> usize {
        self.next.0
    }

    #[inline]
    pub fn nominator(&self) -> usize {
        self.next.1
    }

    pub async fn connect(&mut self) {
        const TIMEOUT : Duration = Duration::from_secs(5);
        for (i, p) in self.players.iter_mut().enumerate() {
            match timeout(TIMEOUT, p.connect()).await {
                Ok(_) => {
                    println!("Successful connection {}", i);
                },
                Err(_) => {
                    println!("Failed connection {}", i);
                },
            }
        }
    }

    pub async fn next_turn(&mut self) -> Result<bool, GameError> {
        let n_id = self.nominator();
        let nominator = self.players.get_mut(n_id).unwrap();
        let nominated_piece = nominator.nominate(&self.board).await;
        self.board.nominate_inplace(nominated_piece)?;

        let p_id = self.placer();
        let placer = self.players.get_mut(p_id).unwrap();
        let placer_position = placer.place(&self.board).await;
        self.board.place_inplace(placer_position)?;

        if self.board.detect_win() {
            return Ok(false);
        }

        self.next = (
            (self.next.0 + 1) % self.connected_players.len(),
            self.next.0,
        );

        Ok(true)
    }

    pub async fn disconnect(&mut self) {
        const TIMEOUT : Duration = Duration::from_secs(5);
        for (i, p) in self.players.iter_mut().enumerate() {
            match timeout(TIMEOUT, p.disconnect()).await {
                Ok(_) => {
                    println!("Successful disconnection {}", i);
                },
                Err(_) => {
                    println!("Failed disconnection {}", i);
                },
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameError {
    QuartoError(QuartoError),
}

impl From<QuartoError> for GameError {
    fn from(value: QuartoError) -> Self {
        GameError::QuartoError(value)
    }
}

pub mod tests {
    use crate::{board::Board, *};

    pub fn play_piece(board: &mut Board, n: usize, r: usize, c: usize) {
        board.nominate_inplace(n).unwrap();
        board
            .place_inplace(Position::from_coord(r, c).unwrap())
            .unwrap();
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
