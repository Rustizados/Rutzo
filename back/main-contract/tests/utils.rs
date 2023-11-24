use gear_lib_old::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::{TokenId, TokenMetadata},
};
use gstd::{ActorId, Encode};
use gtest::{Log, Program, System};
use main_contract_io::*;
use nft_io::*;

pub const MAIN_CONTRACT_ID: u64 = 1;
pub const NFT_CONTRACT_ID: u64 = 2;
pub const CONTRACT_OWNER: u64 = 3;
pub const USERS: &[u64] = &[4, 5, 6];
pub const TOKENS: &[u8] = &[0, 1, 2, 3, 4, 5];

pub fn init_nft_constructor(main_contract: ActorId, name: String, description: String, royalties: Option<Royalties>, max_mint_count: Option<u32>, authorized_minters: Vec<ActorId>) -> InitNFT {
    let collection = Collection {
        name,
        description  
    };
    let constraints = Constraints {
        max_mint_count,
        authorized_minters  
    };
    InitNFT {
        collection,
        royalties,
        constraints,
        main_contract
    }
}

pub fn init_nft_contract(sys: &System, init_message: InitNFT) {
    let nft_contract  = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/nft.wasm"
    );
    let res = nft_contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

pub fn init_main_contract(sys: &System, nft_contract: Option<ActorId>, tokens_metadata_default: Vec<TokenMetadata>) {
    let main_contract = Program::from_file(
        &sys,
        "./../target/wasm32-unknown-unknown/release/main_contract.wasm"
    );
    let init_message=  InitContractData {
        nft_contract,
        tokens_metadata_default
    };
    let res = main_contract.send(
        CONTRACT_OWNER,
        init_message
    );
    
    assert!(!res.main_failed());
}

pub fn default_nfts() -> Vec<TokenMetadata> {
    let mut nfts = Vec::new();
    nfts.push(TokenMetadata {
       name: String::from("Death City Earth"),
        description: String::from("Rock"),
        media: String::from("https://home.rutzo.studio/NFT/death_city_earth.jpg"),
        reference: String::from("20")
    });
    nfts.push(TokenMetadata {
       name: String::from("Chinampa"),
        description: String::from("Water"),
        media: String::from("https://home.rutzo.studio/NFT/chinampa_water.jpg"),
        reference: String::from("25")
    });
    nfts.push(TokenMetadata {
       name: String::from("Chile"),
        description: String::from("Fire"),
        media: String::from("https://home.rutzo.studio/NFT/chile_fire.jpg"),
        reference: String::from("55")
    });
    nfts.push(TokenMetadata {
       name: String::from("peaceful axolotl"),
        description: String::from("Water"),
        media: String::from("https://home.rutzo.studio/NFT/peaceful_axolotl_water.jpg"),
        reference: String::from("33")
    });
    nfts.push(TokenMetadata {
       name: String::from("ixchel"),
        description: String::from("Rock"),
        media: String::from("https://home.rutzo.studio/NFT/ixchel_wind.jpg"),
        reference: String::from("49")
    });
    nfts.push(TokenMetadata {
       name: String::from("tlaloc"),
        description: String::from("Water"),
        media: String::from("https://home.rutzo.studio/NFT/tlaloc_water.jpg"),
        reference: String::from("75")
    });
    nfts
}


pub fn user_information_eq(user: UserDataState, expected: UserDataState) -> bool {
    let UserDataState { current_game, recent_past_game, past_games } = user;
    current_game == expected.current_game && recent_past_game == expected.recent_past_game && past_games == expected.past_games
}

