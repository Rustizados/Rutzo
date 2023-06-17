#![allow(unused)]
#![no_std]

use gstd::{prelude::*, msg, ActorId};
use gear_lib::{non_fungible_token::token::TokenId};

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    CardSelected(TokenId),
    GameFinished {
        winner: ActorId,
        winner_card: TokenId,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    CardDoesNotExist(String),
    FinishSucces
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Scoreboard {
    pub score_p1: u8, 
    pub score_p2: u8,
    pub round: u8
}

#[derive(Encode, Decode, TypeInfo)]
pub struct GameFinishedSt {
    pub winner: ActorId, 
    pub losser: ActorId, 
    pub game_status: GameState
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Gameboard {
    pub player1: ActorId,
    pub player2: ActorId  
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameState {
    Tied,
    Won
}

static mut GAME_SCORE: Option<Scoreboard> = None;
static mut PLAYERS: Option<ActorId> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe { 
        GAME_SCORE = Some(Scoreboard { 
            score_p1: 0,
            score_p2: 0,
            round: 1 
        });
    };
}

#[no_mangle]
extern "C" fn handle() {
    let command: InputMessages = msg::load().expect("Invalid message");
    let command = msg::load().expect("Invalid message");

    let mut game_score = unsafe { GAME_SCORE.take().unwrap() };

    match command {
        InputMessages::GameFinished { winner, winner_card } => {}
        __ => {
            msg::reply(Answer::CardDoesNotExist(String::from("Your don't have that card")), 0).expect("Unable to reply");
        }
    }

    unsafe {
        GAME_SCORE = Some(game_score);
    }
}

#[no_mangle]
extern "C" fn handle_reply() {}

fn player_deck_select() {}

fn player_card_select() {
    let message: InputMessages = msg::load().expect("Invalid message");

    match message {
        InputMessages::CardSelected(token_card) => {
            
            // msg::send(main_id, String::from("Card selected successfully"), 0).expect("Can't send a `SendHelloTo` message");
        }
        __ => {}
    }

}