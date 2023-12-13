use gstd::{prelude::*, ActorId};
use crate::{
    UserDataState,
    UserId,
    MatchState,
    NFTDefault,
    ContractState,
    // MatchInformation,
    NFTOnSale, 
    GameData,
    CardData
};

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateQuery{
    UserIsRegister(UserId),
    UserHasPendingTransfer(UserId),
    CardsFromUserInGame(UserId, u64),
    GameInformationById(u64),
    RoundInformationFromGameId(u64),
    MatchStateById(u64),
    PlayerInformation(UserId),
    PlayerIsInMatch(ActorId),
    NFTsPurchasedByUser(UserId),
    UserCanMintDefaultsNFTs(UserId),
    NFTsOnSale,
    DefaultsNFTS,
    #[default]
    All
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateReply {
    UserIsRegister(bool),
    PendingTransferFrom(Option<u64>),
    CardsInGameFromGivenUser(Option<Vec<CardData>>),
    // GameInformation(MatchInformation),
    GameInformation(GameData),
    RoundState(MatchState),
    PlayerInformation(UserDataState),
    PlayerInMatch(Option<u64>),
    MatchState(MatchState),
    PurchasedNfts(Vec<u64>),
    UserCanMintDefaultsNfts(bool),
    NFTsOnSale(Vec<NFTOnSale>),
    DefaultsNFTs(Vec<NFTDefault>),
    MatchDoesNotExists,
    UserDoesNotExists,
    All(ContractState)
}