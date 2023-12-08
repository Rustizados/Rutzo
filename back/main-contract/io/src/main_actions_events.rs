use gear_lib_old::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{prelude::*, ActorId};
use crate::{
    UserId,
    NFTPrice,
    MatchId,
    DataFromContract
};

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoAction {
    MessageTest,
    Register,
    //Login,
    //Logout,
    PlayGame(TokenId),
    MintCard {
        token_id: u8
    },
    SetNFTAddress(ActorId),
    AddNftForSale {
        token_metadata: TokenMetadata,
        value: u128
    },
    BuyNFT(TokenId),
    ApproveMinter(ActorId),
    DelegateApprovedUser(ActorId),
    SetContractToReceiveInformation(ActorId),
    SetContractToSendData(ActorId),
    RestoreInformationFromOldMainContract,
    GetAllInformation,
    GetProfits,
    DeleteContract,
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoEvent {
    MessageTestResponse,
    Minted(TokenId),
    NFTContractSaved,
    ErrorCallingNFTContract,
    UserIsNotApproved(UserId),
    UserIsNotTheOwner(UserId),
    Approved(UserId),
    UserApprovedNotExists(UserId),
    ApprovedUserDeleted(UserId),
    ErrorBuying(String),
    InsufficientFunds(NFTPrice),
    NftWithTokenIdDoesNotExists(TokenId),
    NFTWithIdNotExists(u8),
    NFTTypeDoesNotExist(String),
    AccountAlreadyExist(UserId),
    AccountNotExists(UserId),
    AccountAlreadyInMatch(UserId),
    QueryNotAllowed(String),
    RegisterSucces,
    LoginSucces,
    UserInMatch(MatchId),
    ErrorInJoiningMatch,
    PurchaseSucces,
    ReplySuccess,
    NewPlayer(UserId),
    MatchFinished,
    MatchCreated,
    UserIsAlreadyInAGame(u64),
    PendingTransfer(TokenId),
    TransferSuccess(TokenId),
    NFTIsNotApprovedByMainContract(TokenId),
    CommunicationError(ActorId),
    MaxMintsReached(UserId),
    Profits(u128),
    ContractSet(ActorId),
    ContractIsNotTheMain,
    InformationRecovered,
    ErrorRetrievingInformation,
    WrongMessageFromNFTContract,
    NFTTypeIsIncorrect((TokenId, String)),
    ContractsData(DataFromContract),
    ContractToReceiveDataNotSet,
}