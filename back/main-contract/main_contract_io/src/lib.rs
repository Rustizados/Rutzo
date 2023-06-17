#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut};
use gear_lib::non_fungible_token::token::TokenId;

pub struct MainContractMetadata;

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
    AccountNotExists(String),
    QueryNotAllowed(String),
    RegisterSucces,
    LoginSucces,
    PurchaseSucces,
    ReplySuccess,
    NewPlayer(ActorId),
    NewPlayerWithMainId(NewPlayerWithIdData)
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewPlayerWithIdData {
    pub main_id: ActorId,
    pub new_player: ActorId
}

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct game_data {
    pub actor_game_id: ActorId,
    pub actual_state: game_state
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum game_state {
    waiting_for_player,
    game_in_progress
}

#[derive(Encode, Decode, TypeInfo)]
pub struct user_games_id {
    pub users: Vec<ActorId>,
    pub games: Vec<game_data>
}


impl Metadata for MainContractMetadata {
    type Init = ();
    type Handle = InOut<InputMessages, Answer>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = user_games_id;
}

