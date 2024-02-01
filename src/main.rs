#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(async_closure)]

use core::panic;
use std::process::exit;

use board::Board;
use player::QuartoPlayer;
use reqwest::{header::HeaderMap, Url};
// use remote::RemoteGame;
use ws::{connect, CloseCode, Handler, Request};

use crate::{game::Game, minimax::MinimaxPlayer, player::RandomPlayer, runner::GameRunner};

use serde_json::{from_str, Value};

use tokio::*;

pub mod board;
pub mod game;
pub mod minimax;
pub mod piece;
pub mod player;
pub mod position;
pub mod runner;
// pub mod remote;

pub const GATEWAY : &'static str = "ws://192.168.0.110:3000/gateway";
pub const GAMES : &'static str = "http://192.168.0.110:3000/games";

#[tokio::main]
async fn main() {
    // let result = pollster::block_on(
    //     GameRunner::new(16, || Game::new(MinimaxPlayer, RandomPlayer)).run(),
    // );
    // println!("{:?}", result);

    // RemoteGame::new(MinimaxPlayer).run()

    connect(GATEWAY, |out| {
        if out.send("KYS").is_err() {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }
        RemoteGame::new(MinimaxPlayer)
    })
    .unwrap();
}

pub struct RemoteGame {
    player: Box<dyn QuartoPlayer>,
    board: Board,
}

impl RemoteGame {
    pub fn new(player: impl QuartoPlayer) -> Self {
        RemoteGame {
            player: Box::new(player),
            board: Board::new(),
        }
    }
}

impl Handler for RemoteGame {
    fn build_request(&mut self, url: &Url) -> ws::Result<ws::Request> {
        let mut req = Request::from_url(url)?;
        let headers = req.headers_mut();
        headers.push(("Bot-Version".to_string(), "0.0".bytes().collect()));
        headers.push(("Bot-Name".to_string(), "Eris".bytes().collect()));
        headers.push((
            "Bot-Concurrency".to_string(),
            1.to_string().bytes().collect(),
        ));
        Ok(req)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let value: Value = serde_json::from_str(msg.as_text()?).unwrap();
        println!("{:?}", msg.to_string());

        let Value::String(s) = value.get("t").unwrap() else {
            panic!()
        };
        match s.as_str() {
            "GAME_CREATE" => {
                let id = value["d"]["game_id"].as_str().unwrap();
                let client = reqwest::Client::new();
                let s = format!("http://192.168.0.110:3000/games/{}/accept", id);
                tokio::spawn(async move {
                    client.post(s)
                    .header("Bot-Name", "Eris")
                    .header("Bot-Version", "0.0")
                    .send().await
                });
            }
            "GAME_CANCEL" => {
                panic!()
            }
            "GAME_END" => {
                exit(1)
            }
            "GAME_UPDATE" => {

            }
            "REQUEST_NOMINATION" => {

            }
            "REQUEST_PLACEMENT" => {
                self.
            }
            _ => ()
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum MakeRequest {
    Create(String),
    // Cancel()
}
