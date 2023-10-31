use gear_lib::non_fungible_token::token;
use gstd::{
    prelude::*, 
    msg, 
    BTreeMap
};
use program_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut CONTRACT: Option<ContractData> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitContractData = msg::load()
        .expect("Error in decoding init message 'InitContractData'");
    unsafe {
        CONTRACT = Some(ContractData {
            owner: msg::source(),
            nft_contract: config.nft_contract,
            tokens_metadata_default: config.tokens_metadata_default
                .into_iter()
                .enumerate()
                .map(|(index, data)| (index as u8, data))
                .collect(),
            ..Default::default()
        });
    };
}

#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    match action {
        RutzoAction::PlayGame {
            token_id,
            power
        } => {
            let message = state.play_game(msg::source(), token_id, power.parse::<u8>().expect("Error parsing")).await;
            msg::reply(message, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::MintCard {
            token_id
        } => {
            msg::reply(state.mint_card(token_id).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetNFTAddress(address) => {
            let user_id = msg::source();
            if user_id != state.owner {
                msg::reply(RutzoEvent::UserIsNotApproved(user_id), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.nft_contract = Some(address);
            
            msg::reply(RutzoEvent::NFTContractSaved, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Register => {
            msg::reply(state.register_user(msg::source()), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },        
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0).expect("Error in sending reply state");
}



pub fn state_mut() -> &'static mut ContractData {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}