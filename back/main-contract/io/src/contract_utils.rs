use gear_lib_old::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{prelude::*, ActorId};
use crate::{
    NFTCardType,
    CardData,
    UserId, 
    contract_types::GameId
};

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserDefaultMints {
    pub nfts_minted: Vec<u8>,
    pub can_mint: bool
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContractData {
    pub nft_contract: Option<ActorId>,
    pub tokens_metadata_default: Vec<TokenMetadata>,
    pub contract_to_receive_data: Option<ActorId>,
    pub contract_to_send_data: Option<ActorId>
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameState {
    WaitigForPlayer,
    GameInProgress
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MatchState {
    Finished {
        winner: UserId,
        loser: UserId
    },
    RoundFinished {
        winner: UserRoundData,
        loser: UserRoundData,
        draw: bool
    },
    #[default]
    InProgress,
    LookingForEnemy,
    NotExists,
    Draw
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MatchInformation {
    pub user_1: UserGameData,
    pub user_2: Option<UserGameData>,
    pub match_state: MatchState,
}

#[derive(Default, Clone)]
pub struct UserData {
    pub current_game: Option<GameId>,
    pub recent_past_game: Option<GameId>,
    pub past_games: Vec<GameId>,
    pub is_logged_id: bool,
    pub session_address: ActorId
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserGameData {
    pub user_id: ActorId,
    pub chosen_nft: TokenId,
    pub nft_type: NFTCardType,
    pub nft_power: u8,
    pub nft_data: TokenMetadata
}




// Para testing, mejorando para partidas de 3 rounds.

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserGameData2 {
    pub user_id: ActorId,
    pub nfts_chosen: Vec<CardData>
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RoundResult {
    pub winner: UserId,
    pub loser: UserId,
    pub is_draw: bool
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserRoundData {
    pub user_id: UserId,
    pub chosen_nft: TokenId
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RoundData {
    pub user1_card: Option<TokenId>,
    pub user2_card: Option<TokenId>
}

#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameData {
    pub user_1: UserGameData2,
    pub user_2: Option<UserGameData2>,
    pub user1_wins: u8,
    pub user2_wins: u8,
    pub round_data: RoundData,
    pub round_state: MatchState,
    pub round: u8,
    pub match_state: MatchState,
    pub playing_with_bot: bool 
}


// hola como estas namin como te fue fkffgabirfrabcfugiehk      


// ------------------------



#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserDataState {
    pub current_game: Option<u64>,  
    pub recent_past_game: Option<u64>,
    pub past_games: Vec<u64>,
    pub session_account: ActorId
}

impl From<UserData> for UserDataState {
    fn from(value: UserData) -> Self {
        let current_game = if let Some(game_id) = value.current_game {
            Some(game_id as u64)
        } else {
            None  
        };
        
        let past_games = value.past_games
            .iter()
            .map(|game_id| *game_id as u64)
            .collect();

        let recent_past_game = if  value.recent_past_game.is_some() {
            let recent_past_game = value.recent_past_game.unwrap();
            Some(recent_past_game as u64)
        } else {
            None
        };

        let session_account = value.session_address;

        Self {
            current_game,
            past_games,
            recent_past_game,
            session_account
        }
    }
}