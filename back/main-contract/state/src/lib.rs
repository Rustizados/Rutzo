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
    
    pub fn matchs_in_progress_information(state: State, token_id: TokenId) -> MatchState {
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
    
    pub fn matchs_in_progress_information(state: State, token_id: TokenId) -> MatchState {
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
    
    pub fn match_finished_information(state: State, token_id: TokenId) -> Vec<MatchState> {
        let match_data = state.games
            .iter()
            .find(|&game| {
                let first = game.user_1.chosen_nft == token_id;
                let second = if let Some(user_data) = &game.user_2 {
                    user_data.chosen_nft == token_id                    
                } else {
                    false
                };
                first || second
            });
        
        match match_data {
            Some(data) => {
                data.match_state.clone()
            },
            None => MatchState::NotExists
        } 
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

