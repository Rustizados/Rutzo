use gstd::{prelude::*, msg, ActorId, Clone};
use gmeta::{Metadata, InOut, Encode, Decode, TypeInfo};
use gear_lib::{non_fungible_token::token::TokenId};

pub struct MatchContractMetadata;

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    // struct idwey y cartas
    CardSelected {
        token_id: TokenId,
        actor_id: ActorId
    },
    NewPlayer(ActorId),
    NewPlayerWithMainId(NewPlayerWithIdData)
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewPlayerWithIdData {
    pub main_id: ActorId,
    pub new_player: ActorId
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    CardDoesNotExist(String),
    FinishSucces
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Scoreboard {
    pub main_contract_id: Option<ActorId>,
    pub score_p1: u8, 
    pub score_p2: u8,
    pub user1_id: Option<ActorId>,
    pub user2_id: Option<ActorId>,

    pub round: u8
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Gameboard {
    pub player1: Player,
    pub player2: Player,
    
    pub game_tie: bool, 
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ConectionState {
    Connected,
    Disconnected
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Player {
    pub player_id: ActorId,
    pub connection_state: ConectionState,
    // pub deck: Vec<TokenId>,
    pub winner: bool,
}

impl Metadata for MatchContractMetadata {
    type Init = ();
    type Handle = InOut<InputMessages, String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Scoreboard;
}