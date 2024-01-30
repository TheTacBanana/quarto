use std::time::Instant;

use futures::future::join_all;

use crate::game::{Game, GameResult};



pub struct GameRunner {
    n : usize,
    runnable : Box<dyn Fn() -> Game>,
}

impl GameRunner {
    pub fn new(n : usize, runnable : impl Fn() -> Game + 'static) -> Self{
        Self {
            n,
            runnable: Box::new(runnable),
        }
    }

    pub async fn run(&mut self) -> RunnerResult {
        let instant = Instant::now();

        let mut games = (0..self.n)
            .into_iter()
            .map(|_| self.runnable.as_mut()())
            .collect::<Vec<_>>();

        let results = join_all(games.iter_mut().map(|x| x.run())).await;
        let win_rate = results.iter().fold((0, 0, 0), |(l, r, d), new| match new {
            Ok(GameResult::Draw) => (l, r, d + 1),
            Ok(GameResult::Win(0)) => (l + 1, r, d ),
            Ok(GameResult::Win(1)) => (l, r + 1, d),
            e => panic!("{:?}", e),
        });

        RunnerResult {
            time_taken: instant.elapsed().as_secs_f32(),
            l_wins: win_rate.0,
            r_wins: win_rate.1,
            draws: win_rate.2,
        }
    }
}

#[derive(Debug)]
pub struct RunnerResult {
    pub time_taken: f32,
    pub l_wins: usize,
    pub r_wins: usize,
    pub draws: usize,
}