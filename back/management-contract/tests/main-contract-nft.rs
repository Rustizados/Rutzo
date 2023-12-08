use gstd::{ActorId, Encode};
use gtest::{Log, Program, System};
use management_io::*;
mod utils;
use utils::*;

#[test]
fn contracts_unavailables_both() {
    let sys = System::new();
    sys.init_logger();
    init_empty(&sys);
    
    let contract = sys.get_program(MANAGEMENT_CONTRACT_ID);
    
    // Read state to check contracts are unavaliables.
    let mut state_query = ManagementStateQuery::ServerIsUnderMaintenance;
    let mut state_message = contract.read_state(state_query);
    
    // Contracts are unavailables
    if let Ok(message_data) = state_message {
        let ManagementStateReply::ServerIsUnderMaintenance(in_maintenance) = message_data else {
            panic!("management state message was incorrect! (ManagementStateReply::ServerIsUnderMaintenance)");  
        };
        
        assert!(in_maintenance, "Wrong state from management contract, it need to be true!");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // Getting data from state, but are unavailable
    let contracts_unavailable = ManagementStateReply::ServersUnavailable;
    let main_contract_suspended = ManagementStateReply::MainContractState(ContractState::Suspended);
    let nft_contract_suspended = ManagementStateReply::NFTContractState(ContractState::Suspended);
    
    // Main Contract state
    state_query = ManagementStateQuery::MainContractState;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, main_contract_suspended, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // Main Contract Data
    state_query = ManagementStateQuery::MainContractData;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // Main Contract Program Id
    state_query = ManagementStateQuery::MainContractProgramId;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // Main Contract Program Id
    state_query = ManagementStateQuery::MainContractMetadata;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // NFT Contract State
    state_query = ManagementStateQuery::NFTContractState;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, nft_contract_suspended, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // NFT ContractData
    state_query = ManagementStateQuery::NFTContractData;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // NFT Contract Program Id
    state_query = ManagementStateQuery::NFTContractProgramId;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
    // NFT Contract Metadata
    state_query = ManagementStateQuery::NFTContractMetadata;
    state_message = contract.read_state(state_query);
    
    if let Ok(state_data) = state_message {
        assert_eq!(state_data, contracts_unavailable, "Incorrect message from state");
    } else {
        panic!("Error in reading management contract state");
    }
    
}

#[test]
fn nft_contract_unavailable_contracts_unavailable() {
    let sys = System::new();
    sys.init_logger();
    init_nft_empty(&sys);
    
    let contract = sys.get_program(MANAGEMENT_CONTRACT_ID);
    
    // Read state to check contracts are unavaliables.
    let state_query = ManagementStateQuery::ServerIsUnderMaintenance;
    let state_message = contract.read_state(state_query);
    
    if let Ok(message_data) = state_message {
        let ManagementStateReply::ServerIsUnderMaintenance(in_maintenance) = message_data else {
            panic!("management state message was incorrect! (ManagementStateReply::ServerIsUnderMaintenance)");  
        };
        
        assert!(!in_maintenance, "Wrong state from management contract, it need to be false!");
    } else {
        panic!("Error in reading management contract state");
    }
}

#[test]
fn main_contract_unavailable_contracts_unavailable() {
    let sys = System::new();
    sys.init_logger();
    init_main_empty(&sys);
    
    let contract = sys.get_program(MANAGEMENT_CONTRACT_ID);
    
    // Read state to check contracts are unavaliables.
    let state_query = ManagementStateQuery::ServerIsUnderMaintenance;
    let state_message = contract.read_state(state_query);
    
    if let Ok(message_data) = state_message {
        let ManagementStateReply::ServerIsUnderMaintenance(in_maintenance) = message_data else {
            panic!("management state message was incorrect! (ManagementStateReply::ServerIsUnderMaintenance)");  
        };
        
        assert!(!in_maintenance, "Wrong state from management contract, it need to be false!");
    } else {
        panic!("Error in reading management contract state");
    }
}
















/*

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

*/