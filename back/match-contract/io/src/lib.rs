#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut, In, Out};
use gear_lib::non_fungible_token::token::TokenId;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<MatchContractData>;
    type Handle = InOut<MatchContractMessage, MatchContractMessage>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<MatchContractData>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractAction {
    NewUser(ActorId),
    ChoosenCard(u8),
    DeleteContract
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractEvent {
    UserAddedToMatch,
    MatchBegin,
    MatchResult(MatchContractResults),
    ContractDeleted,
    QueryNotAllowed(String),
    ErrorAddingUser((ActorId, String))
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewUser {
    user_id: ActorId,
}




// Para test   -----------------------------
/*
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractData {
    pub mainContractId: ActorId
}
*/

#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractResults {
    pub data: String,
}


#[derive(Encode, Decode, TypeInfo)]
pub enum RutzoAction {
    Login,
    Register,
    PlayGame,
    QuitGame,
    GetCards,
    BuyCard,
    EndGame,
    TransferCard {
        from: ActorId,
        to: ActorId,
        matchId: ActorId
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

