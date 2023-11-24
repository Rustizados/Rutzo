use gear_lib_old::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::{TokenId, TokenMetadata},
};
use gstd::{ActorId, Encode};
use gtest::{Log, Program, System};
use nft_io::*;
use main_contract_io::*;
use utils::{init_main_contract, NFT_CONTRACT_ID, MAIN_CONTRACT_ID};
mod utils;
use utils::*;

#[test]
fn init_main_contract_fail() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, None, default_nfts());
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    
    // must fail since the main contract does not have the nft contract address
    
    let res = main_contract.send(
        USERS[0],
        RutzoAction::Register
    );
    
    assert!(res.main_failed());
}


#[test]
fn init_main_contract_correct() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, Some(NFT_CONTRACT_ID.into()), default_nfts());
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    
    let res = main_contract.send(
        USERS[0],
        RutzoAction::Register
    );
    
    assert!(!res.main_failed());
}


#[test]
fn register_main_contract() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, Some(NFT_CONTRACT_ID.into()), default_nfts());
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    
    let res = main_contract.send(
        USERS[0],
        RutzoAction::Register
    );
    
    let log = Log::builder()
        .dest(USERS[0])
        .payload(RutzoEvent::RegisterSucces);
    
    assert!(!res.main_failed());
    assert!(res.contains(&log));
    
    let state_query = RutzoStateQuery::PlayerInformation(USERS[0].into());
    let state_message = main_contract.read_state(state_query);
    let message = UserDataState {   
        current_game: None,  
        recent_past_game: None,
        past_games: vec![],
    };
    
    if let Ok(message_data) = state_message {
       let RutzoStateReply::PlayerInformation(user_info) = message_data else {
           panic!("State message was incorrect 'RutzoStateReply::PlayerInformation'");  
        };
        if !user_information_eq(user_info, message) {
            panic!("Information info is not correct!");
        }
    } else {
        panic!("Error in reading state from main contract");
    }
}


#[test]
fn register_fail_main_contract() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, Some(NFT_CONTRACT_ID.into()), default_nfts());
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    
    // State must fail since the user does not exists
    
    let state_query = RutzoStateQuery::PlayerInformation(USERS[0].into());
    let state_message = main_contract.read_state(state_query);
    
    if let Ok(message_data) = state_message {
        if let RutzoStateReply::UserDoesNotExists = message_data {} else {
            panic!("Wrong message type");
        } 
    } else {
        panic!("Error in reading state from main contract");
    }
    
    // must fail since the user is not registered
    
    let mut res = main_contract.send(
        USERS[0], 
        RutzoAction::PlayGame(4.into())
    );
    
    let mut log = Log::builder()
        .dest(USERS[0])
        .payload(RutzoEvent::AccountNotExists(USERS[0].into()));
        
    assert!(!res.main_failed());
    assert!(res.contains(&log));
    
    // Correct register
    
    res = main_contract.send(
        USERS[0],
        RutzoAction::Register
    );
    
    log = Log::builder()
        .dest(USERS[0])
        .payload(RutzoEvent::RegisterSucces);
    
    assert!(!res.main_failed());
    assert!(res.contains(&log));
    
    // must fail since the user is already subscribed
    
    res = main_contract.send(
        USERS[0],
        RutzoAction::Register
    );
    
    log = Log::builder()
        .dest(USERS[0])
        .payload(RutzoEvent::AccountAlreadyExist(USERS[0].into()));
    
    assert!(!res.main_failed());
    assert!(res.contains(&log));
    
}


#[test]
fn mint_default_nft() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, Some(NFT_CONTRACT_ID.into()), default_nfts());
    let nft_init_message = init_nft_constructor(
        MAIN_CONTRACT_ID.into(), 
        String::from("Rutzo"), 
        String::from("Rutzo nft"), 
        None, 
        None, 
        vec![MAIN_CONTRACT_ID.into()]
    );
    init_nft_contract(&sys, nft_init_message);
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    let nft_contract = sys.get_program(NFT_CONTRACT_ID);
    
    let mut res = main_contract.send(
        USERS[0], RutzoAction::Register
    );
    
    assert!(!res.main_failed());
    
    res = main_contract.send(
        USERS[0], 
        RutzoAction::MintCard { 
            token_id: TOKENS[0] 
        }
    );
    
    // first nft minted
    
    let mut log = Log::builder()
        .dest(USERS[0])
        .payload(RutzoEvent::Minted(0.into()));
    
    assert!(res.contains(&log));
        
    res = nft_contract.send(
        MAIN_CONTRACT_ID, 
        NFTAction::NFTData(0.into())
    );
        
    log = Log::builder()
        .dest(MAIN_CONTRACT_ID)
        .payload(NFTEvent::NFTData(Some(TokenMetadata {
            name: String::from("Death City Earth"),
            description: String::from("Rock"),
            media: String::from("https://home.rutzo.studio/NFT/death_city_earth.jpg"),
            reference: String::from("20")
        })));
    
    assert!(!res.main_failed());
    assert!(res.contains(&log));
    
}


#[test]
fn mint_default_nft_fail() {
    let sys = System::new();
    sys.init_logger();
    init_main_contract(&sys, Some(NFT_CONTRACT_ID.into()), default_nfts());
    let nft_init_message = init_nft_constructor(
        MAIN_CONTRACT_ID.into(), 
        String::from("Rutzo"), 
        String::from("Rutzo nft"), 
        None, 
        None, 
        vec![MAIN_CONTRACT_ID.into()]
    );
    init_nft_contract(&sys, nft_init_message);
    let main_contract = sys.get_program(MAIN_CONTRACT_ID);
    let nft_contract = sys.get_program(NFT_CONTRACT_ID);
    
    let mut res = main_contract.send(
        USERS[0], RutzoAction::Register
    );
    
    assert!(!res.main_failed());
    
    
}

