#![no_std]

use program_io::*;
use gear_lib::non_fungible_token::token::TokenId;
use gear_lib::non_fungible_token::{
    token::*
};
use gstd::{prelude::*, ActorId};

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[gmeta::metawasm]
pub mod metafns {

    pub type State = ContractData;

    pub fn is_register(state: State, user_id: ActorId) -> bool {
       state.is_register(&user_id)
    }
    
    pub fn match_in_progress_by_id(state: State, token_id: TokenId) -> MatchState {
        for game in state.games.iter() {
            let first = game.user_1.chosen_nft == token_id;
            let second = if let Some(user_data) = &game.user_2 {
                user_data.chosen_nft == token_id                    
            } else {
                false
            };
            
            if first || second {
                if let MatchState::InProgress  = game.match_state {
                    return game.match_state.clone();
                }
            }
        }
        MatchState::NotExists
    }
    
    pub fn matchs_finished_by_id(state: State, token_id: TokenId) -> Vec<MatchState> {
        let mut finished_matchs = Vec::new();
        for game in state.games.iter() {
            let first = game.user_1.chosen_nft == token_id;
            let second = if let Some(user_data) = &game.user_2 {
                user_data.chosen_nft == token_id                    
            } else {
                false
            };
            
            if first || second {
                if let MatchState::InProgress  = game.match_state {
                    finished_matchs.push(game.match_state.clone());
                }
            }
        }
        finished_matchs
    }
    
    pub fn matchs_finished_by_id_and_user(state: State, token_id: TokenId, address: ActorId) -> Vec<MatchState> {
        let mut finished_matchs = Vec::new();
        for game in state.games.iter() {
            let user1 = game.user_1.user_id;
            let user2 = if let Some(user_data) = &game.user_2 {
                user_data.user_id.clone()               
            } else {
                ActorId::default()
            };
            let first = game.user_1.chosen_nft == token_id;
            let second = if let Some(user_data) = &game.user_2 {
                user_data.chosen_nft == token_id                    
            } else {
                false
            };
            
            if (first || second) && (user1 == address || user2 == address) {
                if let MatchState::InProgress  = game.match_state {
                    finished_matchs.push(game.match_state.clone());
                }
            }
        }
        finished_matchs
    }
    
    pub fn nfts_availables(state: State, user_id: ActorId) -> Vec<(Option<u8>, TokenMetadata)> {
        let user_mints = state.default_tokens_minted_by_id.get(&user_id);
        
        match user_mints {
            Some(mints) => {
                if mints.len() > 3 {
                    return state.tokens_metadata_default
                        .iter()
                        .map(|(token_id, token_metadata)| (None, token_metadata.clone()))
                        .collect();
                }
                state.tokens_metadata_default
                    .iter()
                    .map(|(token_id, token_metadata)| {
                        if mints.iter().find(|&id| id == token_id).is_none() {
                            (Some(*token_id), token_metadata.clone())
                        } else {
                            (None, token_metadata.clone())
                        }
                    })
                    .collect()
            },
            None => {
                state.tokens_metadata_default
                    .iter()
                    .map(|(token_id, token_metadata)| (Some(*token_id), token_metadata.clone()))
                    .collect()
            }
        }
    }
}

