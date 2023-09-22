#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut, In};
use gear_lib::non_fungible_token::token::TokenId;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<InputMessages, Answer>;
    type Reply = In<MatchContractMessage>;
    type Others = ();
    type Signal = ();
    type State = UserGamesId;
}

// State: Beta -------------------------
#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractMessage {
    NewUser(ActorId),
    MatchResult(MatchContractResults),
    AcceptedUser,
    DeleteContract,
    QueryNotAllowed(String),
    ErrorAddingUser((ActorId, String))
}

//  State: Beta   -----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractData {
    pub first_user: Option<ActorId>,
    pub second_user: Option<ActorId>,
    pub mainContractId: ActorId
}

// State: Beta ----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractResults {
    pub data: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
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
    },
    DeleteMainContract,  // test----------------
    DeleteMainContractSecond,
    DeleteContracts
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
    NewPlayerWithMainId(NewPlayerWithIdData),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewPlayerWithIdData {
    pub main_id: ActorId,
    pub new_player: ActorId
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct GameData {
    pub actor_game_id: ActorId,
    pub actual_state: GameState
}

#[derive(Decode, Encode, TypeInfo)]
pub struct UserData {
    pub user_id: ActorId,
    pub in_game: bool
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum GameState {
    WaitigForPlayer,
    GameInProgress
}

#[derive(Encode, Decode, TypeInfo)]
pub struct UserGamesId {
    pub users: Vec<UserData>,
    pub games: Vec<GameData>,
}
