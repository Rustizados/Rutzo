#![allow(unused)]
#![no_std]

use gstd::{prelude::*, msg, ActorId};
use gear_lib::non_fungible_token::token::TokenId;

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    GameFinished {
        winner: ActorId,
        winner_card: TokenId,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    FinishSucces
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Scoreboard {
    pub score_p1: u8, 
    pub score_p2: u8,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct GameFinished {
    pub winner: ActorId, 
    pub losser: ActorId, 
    pub game_status: GameState
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameState {
    Tied,
    Won
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Signvdx {
    UserId,
    GameId
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TypeId {
    UserId,
    GameId
}

/* #[derive(Encode, Decode, TypeInfo)]
pub enum TypeId {
    UserId,
    GameId
} */

#[no_mangle]
extern "C" fn init() {}

#[no_mangle]
extern "C" fn handle() {}

#[no_mangle]
extern "C" fn handle_reply() {}