#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut, In};
use gear_lib::non_fungible_token::token::TokenId;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<MatchContractData>;
    type Handle = InOut<MatchContractMessage, MatchContractMessage>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = MatchContractData;
}

// Para test -------------------------
#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractMessage {
    NewUser(ActorId),
    MatchResult(MatchContractResults),
    AcceptedUser,
    DeleteContract,
    QueryNotAllowed(String),
    ErrorAddingUser((ActorId, String))
}

// Para test   -----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractData {
    pub first_user: Option<ActorId>,
    pub second_user: Option<ActorId>,
    pub mainContractId: ActorId
}

// Para test ----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractResults {
    pub data: String,
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
    ContractCreated(MatchContractResults), // para test -------------------------------
    CleanContractCreatedData
}

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

