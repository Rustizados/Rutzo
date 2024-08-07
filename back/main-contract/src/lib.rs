#![no_std]

use gstd::{
    prelude::*, 
    msg ,
    exec
};
use main_contract_io::{
    contract_utils::InitContractData,
    main_actions_events::{
        RutzoAction,
        RutzoEvent
    },
    main_state_actions_events::{
        RutzoStateQuery,
        RutzoStateReply 
    },
    nft_utils::{
        NFTOnSale,
        NFTDefault, CardData
    },
    contract_types::ONE_TVARA_VALUE,
    Contract
};

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitContractData = msg::load()
        .expect("Error in decoding init message 'InitContractData'");
    unsafe {
        CONTRACT = Some(Contract {
            owner: msg::source(),
            nft_contract: config.nft_contract,
            tokens_metadata_default: config.tokens_metadata_default
                .into_iter()
                .enumerate()
                .map(|(index, data)| (index as u8, data))
                .collect(),
            accept_data_from: config.contract_to_receive_data,
            send_data_to: config.contract_to_send_data,
            ..Default::default()
        });
    };
}

#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    
    let mut set_nft_contract = false;
    if let RutzoAction::SetNFTAddress(_) = action {
        set_nft_contract = true;
    }
    
    if !set_nft_contract && state.nft_contract.is_none() {
        panic!("Nft contract does not exists!");
    }
    
    let caller = msg::source();
    
    match action {
        RutzoAction::MessageTest => {
            msg::reply(RutzoEvent::MessageTestResponse(exec::block_timestamp()), 0) 
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::PlayGame {
            token_id,
            play_with_bot
        } => {
            //let message = state.play_game(msg::source(), token_id, power.parse::<u8>().expect("Error parsing")).await;
            let message = state.play_game(caller, token_id, play_with_bot).await;
            msg::reply(message, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::MintCard {
            token_id
        } => {
            msg::reply(state.mint_default_card(token_id).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetNFTAddress(address) => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotApproved(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.nft_contract = Some(address);
            
            msg::reply(RutzoEvent::NFTContractSaved, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Register { user_address } => {
            // Trick function, this logic is for signless account, this works for
            // wallets that does not have any tokens
            
            msg::reply(state.register_user(user_address, caller), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Login => {
            msg::reply(state.login_user(caller), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Logout => {
            msg::reply(state.logout_user(caller), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::AddNftForSale { 
            token_metadata,
            value 
        } => {
            msg::reply(state.mint_nft_to_sale(caller, token_metadata, value).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::BuyNFT(token_id) => {
            let value = msg::value();
            let (message, value_to_return) = state.buy_nft(
                caller, 
                token_id, 
                value
            ).await;
            
            msg::reply(message, value_to_return)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::ApproveMinter(user_id) => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.approved_minters.push(user_id);
            
            msg::reply(RutzoEvent::Approved(user_id), 0)
                 .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::DelegateApprovedUser(user_id) => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            let index = match state.approved_minters
                .iter()
                .enumerate()
                .find(|&(_, approved_user)| *approved_user == user_id) {
                    Some((index, _)) => {
                        index
                    },
                    None => {
                        msg::reply(RutzoEvent::UserApprovedNotExists(user_id), 0)
                            .expect("Error in reply a message 'RutzoEvent'");
                        return;
                    }
                };
                
            state.approved_minters.swap_remove(index);
            
            msg::reply(RutzoEvent::ApprovedUserDeleted(user_id), 0) 
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetContractToReceiveInformation(contract_id) => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.accept_data_from = Some(contract_id);
            
            msg::reply(RutzoEvent::ContractSet(contract_id), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetContractToSendData(contract_address) => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.send_data_to = Some(contract_address);
            
            msg::reply(RutzoEvent::ContractSet(contract_address), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::RestoreInformationFromOldMainContract => {
            msg::reply(state.restore_data(caller).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::GetAllInformation => {
            msg::reply(state.all_data(msg::source()).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::GetProfits => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            if state.contract_balance > 10* ONE_TVARA_VALUE {
                let profit = (state.contract_balance) / ONE_TVARA_VALUE ;
                state.contract_balance = 0;
                msg::reply(RutzoEvent::Profits(profit), profit)
                    .expect("Error sending profits to owner!");
            } else {
                msg::reply(RutzoEvent::InsufficientFunds(state.contract_balance), 0)
                    .expect("Error sending profits to owner!");
            }
        },
        RutzoAction::DeleteContract => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            } 
            
            exec::exit(caller);
        },
        RutzoAction::JoinGame {
            cards_id,
            play_with_bot
        } => {
            let response = state.join_game(caller, cards_id, play_with_bot).await;
            msg::reply(response, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::ThrowCard(card_id) => {
            let response = state.throw_card(caller, card_id).await;
            msg::reply(response, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SendNFTToWinner(nft_id) => {
            let response = state.send_nft_to_winner(caller, nft_id).await;
            msg::reply(response, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::DeleteAllData => {
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.games2 = Vec::new();
            state.games_waiting = Vec::new();
            state.games_information_by_user.clear();
            state.game_id = 0;
            state.default_tokens_minted_by_id.clear();
            state.pending_transfers.clear();
        }
    }
}

#[no_mangle]
unsafe extern "C" fn state() {
    let contract= state_ref();
    let message = msg::load()
        .expect("Error in decode 'RutzoStateAction'");
    match message {
        RutzoStateQuery::TotalUsersOnline => {
            msg::reply(RutzoStateReply::UsersOnline(contract.users_online), 0)
                        .expect("Error in decode 'RutzoStateReply'");
        },
        RutzoStateQuery::UserIsRegister(user_address)=> {
            let is_register = contract.games_information_by_user.contains_key(&user_address);
            msg::reply(RutzoStateReply::UserIsRegister(is_register), 0)
                        .expect("Error in decode 'RutzoStateReply'");
        },
        RutzoStateQuery::UserHasPendingTransfer(user_id) => {
            let game_id_pending_transfer = if let Some((_, game_id)) = contract.pending_transfers.get(&user_id) {
                Some(*game_id as u64)
            } else  {
                None
            };
            
            msg::reply(RutzoStateReply::PendingTransferFrom(game_id_pending_transfer), 0)
                        .expect("Error in decode 'RutzoStateReply'");
        },
        RutzoStateQuery::CardsFromUserInGame(user_id, game_id) => {
            let game_id = game_id as usize;
            match contract.games2.get(game_id) {
                Some(data) => {
                    let cards: Option<Vec<CardData>>;
                    if data.user_1.user_id == user_id {
                        cards = Some(data.user_1.nfts_chosen.clone());
                    } else if data.user_2.as_ref().unwrap().user_id == user_id {
                        cards = Some(data.user_2.as_ref().unwrap().nfts_chosen.clone());
                    } else {
                        cards = None;
                    }
                    msg::reply(RutzoStateReply::CardsInGameFromGivenUser(cards), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::MatchDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            };
        },
        RutzoStateQuery::GameInformationById(game_index) => {
            let game_information = contract.games2.get(game_index as usize);
            match game_information {
                Some(data) => {
                    msg::reply(RutzoStateReply::GameInformation(data.clone()), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::MatchDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::RoundInformationFromGameId(game_index) => {
            let game_information = contract.games2.get(game_index as usize);
            match game_information {
                Some(data) => {
                    msg::reply(RutzoStateReply::RoundState(data.round_state.clone()), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::MatchDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::PlayerInformation(user_address) => {
            let user_data = contract.games_information_by_user.get(&user_address);
            match user_data {
                Some(data) => {
                    msg::reply(RutzoStateReply::PlayerInformation((*data).clone().into()), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::UserDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::PlayerIsInMatch(user_address) => {
            let user_data = contract.games_information_by_user.get(&user_address);
            match user_data {
                Some(data) => {
                    let information = if let Some(game_id) = data.current_game {
                        Some(game_id as u64)
                    } else {
                        None
                    };
                    msg::reply(RutzoStateReply::PlayerInMatch(information), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::UserDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::MatchStateById(game_id) => {
            match contract.games2.get(game_id as usize) {
                Some(game_data) => {
                    msg::reply(RutzoStateReply::MatchState(game_data.match_state.clone()), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::MatchDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::NFTsPurchasedByUser(user_address) => {
            match contract.default_tokens_minted_by_id.get(&user_address) {
                Some(nfts) => {
                    let sales_nfts = nfts
                        .nfts_minted
                        .iter()
                        .map(|nft| *nft as u64)
                        .collect();
                    msg::reply(RutzoStateReply::PurchasedNfts(sales_nfts), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::UserDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::UserCanMintDefaultsNFTs(user_address) => {
              match contract.default_tokens_minted_by_id.get(&user_address) {
                Some(nfts) => {
                    let mut can_mint = true;
                    if !nfts.can_mint {
                        can_mint = false;
                    }
                    msg::reply(RutzoStateReply::UserCanMintDefaultsNfts(can_mint), 0)
                        .expect("Error in decode 'RutzoStateReply'");
                },
                None => {
                    msg::reply(RutzoStateReply::UserDoesNotExists, 0)
                        .expect("Error in decode 'RutzoStateReply'");
                }
            }
        },
        RutzoStateQuery::DefaultsNFTS => {
            let defaults_nfts = contract.tokens_metadata_default
                .iter()
                .map(|(index_of_nft, token_metadata)| {
                    let nft_default = NFTDefault {
                        name:   token_metadata.name.clone(),
                        description: token_metadata.description.clone(),
                        media: token_metadata.media.clone(),
                        reference: token_metadata.reference.clone(),
                        sale_id: *index_of_nft as u64
                    };
                    nft_default
                })
                .collect();
            msg::reply(RutzoStateReply::DefaultsNFTs(defaults_nfts), 0)
                .expect("Error in decode 'RutzoStateReply'");
        },
        RutzoStateQuery::NFTsOnSale => {  
            let nfts_on_sale = contract.nfts_for_sale
                .iter()
                .map(|(token_id, value)| {
                    NFTOnSale {
                        token_id: token_id.clone(),
                        value: *value
                    }
                })
                .collect();
            
            msg::reply(RutzoStateReply::NFTsOnSale(nfts_on_sale), 0)
                .expect("Error in decode 'RutzoStateReply'");
        },
        RutzoStateQuery::All => {
             let contract = CONTRACT.take().expect("Unexpected error in taking state");
            msg::reply(RutzoStateReply::All(contract.into()), 0)
                .expect("Error in sending reply state");
        }
    }
}


pub fn state_mut() -> &'static mut Contract {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

pub fn state_ref() -> &'static Contract {
    let state = unsafe { CONTRACT.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}


/*
{
    "name": "Death City Earth",
    "description": "Rock",
    "media": "https://home.rutzo.studio/NFT/rock/zacualpan_rock.jpg",
    "reference": "20"
},
{
    "name": "Chinampa",
    "description": "Water",
    "media": "https://home.rutzo.studio/NFT/water/chinampa_water.jpg",
    "reference": "25"
},
{
    "name": "Chile",
    "description": "Fire",
    "media": "https://home.rutzo.studio/NFT/fire/chile_fire.jpg",
    "reference": "55"
},
{
    "name": "peaceful axolotl",
    "description": "Water",
    "media": "https://home.rutzo.studio/NFT/water/ajolote_water.jpg",
    "reference": "33"
},
{
    "name": "ixchel",
    "description": "Ice",
    "media": "https://home.rutzo.studio/NFT/ice/ixchel_ice.jpg",
    "reference": "33"
},
{
    "name": "tlaloc",
    "description": "Ice",
    "media": "https://home.rutzo.studio/NFT/ice/tlaloc_ice.jpg",
    "reference": "75"
}
*/