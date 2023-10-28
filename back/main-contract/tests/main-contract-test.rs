mod utils;

use gtest::{Log, System, Program};
use program_io::*;


const USERS: &[u64] = &[4, 5, 6, 7, 8, 9];
const NFT_CONTRACT: u64 = 2;
const MAIN_CONTRACT: u64 = 1;

pub struct InitContractData {
    pub match_contract: String,
    pub nft_contract: ActorId,
}


#[test]
fn contract_init() {
    let sys = System::new();
    init_contract(&sys);
    let contract = sys.get_program(1);
}

fn init_contracts(sys: &System) {
    sys.init_logger();
    let contract = Program::current(&sys);
    let nft = Program::from_file(&sys, "../../nft-contract/target/wasm32-unknown-unknown/release/main_contract.wasm");
    
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    
    res = contract.send(USERS[0], InitContractData {
        match_contract: String::from("sfasdfsf09sdf90afdjasasdf"),
        nft_contract: NFT_CONTRACT.into()
    });
    assert!(!res.main_failed());
}