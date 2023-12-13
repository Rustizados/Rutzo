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
    PlayGame {
        token_id: TokenId,
        play_with_bot: bool
    },
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
    
    // For testing the matchs with three rounds
    
    JoinGame {
        cards_id: Vec<TokenId>,
        play_with_bot: bool
    },
    ThrowCard(TokenId),
    SendNFTToWinner(TokenId),
    DeleteAllData
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoEvent {
    MessageTestResponse(u64),
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
    PurchaseSucces,
    ReplySuccess,
    NewPlayer(UserId),
    PendingTransfer,
    UserDoesNotHasPendingTransfer,
    NftIdIsNotInGamId {
        nft_id: TokenId,
        game_id: u64
    },
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
    ContractsData(DataFromContract),
    ContractToReceiveDataNotSet,
    
    // Game and Rounds events
    BotWinRound,
    PlayerOneWin,
    PlayerTwoWin,
    RoundDraw,
    UserInMatch(MatchId),
    UserIsNotInAGame(UserId),
    GameIdDoesNotExists(u64),
    GameIsWaitingPlayer(u64),
    UserAlreadyChoseACard,
    UserHasNotYetSelectedACard(UserId),
    ErrorInJoiningMatch,
    NFTTypeIsIncorrect((TokenId, String)),  
    TotalTokensIdIsIncorrect(Vec<TokenId>),
    UserIsAlreadyInAGame(u64),  
    RoundFinished(u8),
    MatchFinished(u64),
    MatchCreated,
    MatchJoined,
    UserChoseCard,   
    NFTSelectedIsNotInCardsSelected(TokenId),    
}