use gstd::{ActorId, Encode};
use gtest::{Log, Program, System};
use management_io::*;

pub const MANAGEMENT_CONTRACT_ID: u64 = 1;
pub const MAIN_CONTRACT_ID: u64 = 2;
pub const NFT_CONTRACT_ID: u64 = 3;
pub const CONTRACT_OWNER: u64 = 4;
pub const USERS: &[u64] = &[5, 6, 7];

pub fn init_empty(sys: &System) {
    let contract  = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/management_contract.wasm"
    );
    
    let init_message = InitContract {
        main_contract_id: None,
        main_contract_metadata: None,
        nft_contract_id: None,
        nft_contract_metadata: None
    };
    
    let res = contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

pub fn init_main_empty(sys: &System) {
    let contract  = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/management_contract.wasm"
    );
    
    let init_message = InitContract {
        main_contract_id: None,
        main_contract_metadata: None,
        nft_contract_id: Some(NFT_CONTRACT_ID.into()),
        nft_contract_metadata: Some(String::from("0203040230fsd0f0230f0c202342394203482093sdfwefwf30v0sdjc0df9j203j9fs0d"))
    };
    
    let res = contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

pub fn init_nft_empty(sys: &System) {
    let contract  = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/management_contract.wasm"
    );
    
    let init_message = InitContract {
        main_contract_id: Some(MAIN_CONTRACT_ID.into()),
        main_contract_metadata: Some(String::from("0203040230fsd0f0230f0c2030v0sdjc0df9j203j9fs0d")),
        nft_contract_id: None,
        nft_contract_metadata: None
    };
    
    let res = contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

pub fn init_contract(sys: &System) {
    let contract  = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/management_contract.wasm"
    );
    
    let init_message = InitContract {
        main_contract_id: Some(MAIN_CONTRACT_ID.into()),
        main_contract_metadata: Some(String::from("0203040230fsd0f0230f0c2030v0sdjc0df9j203j9fs0d")),
        nft_contract_id: Some(NFT_CONTRACT_ID.into()),
        nft_contract_metadata: Some(String::from("0203040230fsd0f0230f0c202342394203482093sdfwefwf30v0sdjc0df9j203j9fs0d"))
    };
    
    let res = contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

