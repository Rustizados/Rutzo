use gear_lib_old::non_fungible_token::token::{TokenId, TokenMetadata};
use gstd::{prelude::*, ActorId};
use crate::NFTCardType;

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
        winner: ActorId,
        loser: ActorId
    },
    #[default]
    InProgress,
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
    pub current_game: Option<usize>,
    pub recent_past_game: Option<usize>,
    pub past_games: Vec<usize>,
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

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserDataState {
    pub current_game: Option<u64>,  
    pub recent_past_game: Option<u64>,
    pub past_games: Vec<u64>,
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
        Self {
            current_game,
            past_games,
            recent_past_game
        }
    }
}