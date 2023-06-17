#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut};
use gear_lib::non_fungible_token::token::TokenId;

pub struct MainContractMetadata;

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    Login(id_type),
    Register(id_type),
    PlayGame(id_type),
    QuitGame(id_type),
    GetCards(id_type),
    BuyCard(TokenId),
    EndGame(id_type),
    TransferCard {
        from: ActorId,
        to: ActorId,
        matchId: ActorId
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    ErrorBuying(String),
    AccountAlreadyExist(String),
    RegisterSucces,
    LoginSucces,
    PurchaseSucces,
}

#[derive(TypeInfo)]
pub struct user_games_id {
    pub users: Vec<ActorId>,
    pub games: Vec<ActorId>
}

impl Metadata for MainContractMetadata {
    type Init = ();
    type Handle = InOut<InputMessages, Answer>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = user_games_id;
}
