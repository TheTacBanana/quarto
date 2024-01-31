use std::time::Duration;

use async_std::future::{timeout, TimeoutError};
use futures::future::join_all;

use crate::{
    board::{Board, QuartoError},
    player::QuartoPlayer,
};

pub struct Game {
    board: Board,
    players: [Box<dyn QuartoPlayer>; 2],
    next: u8,
}

impl Game {
    pub const CONN_TIMEOUT: Duration = Duration::from_secs(5);

    pub fn new(p1: impl QuartoPlayer, p2: impl QuartoPlayer) -> Self {
        Game {
            players: [Box::new(p1), Box::new(p2)],
            board: Board::new(),
            next: 0b10,
        }
    }

    pub async fn run(&mut self) -> Result<GameResult, GameError> {
        pollster::block_on(self.connect())?;

        loop {
            match pollster::block_on(self.next_turn())? {
                GameState::Finished(res) => {
                    println!("{:?}", self.board);
                    pollster::block_on(self.disconnect())?;
                    return Ok(res)
                }
                GameState::Continue => (),
            }
        }
    }

    pub async fn connect(&mut self) -> Result<(), GameError> {
        timeout(
            Game::CONN_TIMEOUT,
            join_all(self.players.iter_mut().map(|x| x.connect())),
        )
        .await?
        .iter()
        .all(|x| x.is_ok())
        .then(|| ())
        .ok_or(GameError::FailedConnection)
    }

    #[inline]
    pub fn nominator(&self) -> usize {
        self.next.trailing_zeros() as usize
    }

    #[inline]
    pub fn placer(&self) -> usize {
        self.next.trailing_ones() as usize
    }

    pub async fn next_turn(&mut self) -> Result<GameState, GameError> {
        let n_id = self.nominator();
        let nominator = self.players.get_mut(n_id).unwrap();
        let nominated_piece = timeout(Game::CONN_TIMEOUT, nominator.nominate(&self.board)).await?;
        self.board.nominate_inplace(nominated_piece)?;

        let p_id = self.placer();
        let placer = self.players.get_mut(p_id).unwrap();
        let placer_position = timeout(Game::CONN_TIMEOUT, placer.place(&self.board)).await?;
        self.board.place_inplace(placer_position)?;

        if self.board.detect_win() {
            return Ok(GameState::Finished(GameResult::Win(self.placer())));
        }

        self.next ^= 3;

        if self.board.piece_bits() == 0 {
            Ok(GameState::Finished(GameResult::Draw))
        } else {
            Ok(GameState::Continue)
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), GameError> {
        timeout(
            Game::CONN_TIMEOUT,
            join_all(self.players.iter_mut().map(|x| x.connect())),
        )
        .await?
        .iter()
        .all(|x| x.is_ok())
        .then(|| ())
        .ok_or(GameError::FailedConnection)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Finished(GameResult),
    Continue,
}

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Win(usize),
    Draw,
}

#[derive(Debug, Clone, Copy)]
pub enum GameError {
    QuartoError(QuartoError),
    FailedConnection,
    BadDisconnect,
    ConnectionTimeout,
}

impl From<TimeoutError> for GameError {
    fn from(_value: TimeoutError) -> Self {
        GameError::ConnectionTimeout
    }
}

impl From<QuartoError> for GameError {
    fn from(value: QuartoError) -> Self {
        GameError::QuartoError(value)
    }
}

pub mod tests {
    use crate::{board::Board, position::Position};

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
