use gear_lib::non_fungible_token::{state::*, token::*};
use gstd::{prelude::*, ActorId};
use gtest::{Program, RunResult, System};
use program_io::*;
use program_state::WASM_BINARY;

const USERS: &[u64] = &[3, 4, 5];
const MARKETPLACE_ID: u64 = 6;
const MAINCONTRACT_ID: u64 = 7;
    
pub fn init_nft_no_marketplace(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);
    
    let nfts = nfts_tests();
    let default_nfts = default_nfts_tests();
    
    let res = nft.send(
        USERS[0],
        InitNFT {
            name: String::from("RutzoOnChainToken"),
            symbol: String::from("ROCT"),
            base_uri: String::from(""),
            royalties: None,
            base_image: String::from("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAAAYCAYAAAAMAljuAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAVhSURBVHgB7VrdUttGFD4rCaYXdHDewJ4AM70q3HVaIOIJoE+AeQLgtpgiF9Nb8BNgniDkCXAwzeQu3GUmkMGP4JnmohNJuz1HK8mrHxtJdlLT+JshslbnSKv99vwqAFNMFFgWIfPBKnFu7AoBe3JEXNq6W39bsbowxVjxKCHrHxtHPhGl6BXRBcGancXaGUwxNgwkZPXu2GSMvQSfCAHQdmxn5zv87eqGBRpsS0nRZaBb1wu/XcCEgSy7XbF68ISQIOTnDyfLuiZO8aepDPds3VkJXFScLB/oxpz9LG4M9S3ICOOTe9ZekYuq6qnjKfen+aFli9c3i4fWL3eNPQaiBDngOG5LN4ytvHoEjekPwQZd/XiyC5w/yypvqBfQPZ3iS/hxAnqMiwuhaZtoBeUZ13hYuz9uAWcMadwOZFC+yZiG52ILZbbW7xpn14u1/WEToMWCjHDmdDpYcb1/StCi58flV+8aRzhDyz99LV+Y7eKhDJkhum9/sKy1+5OcelL3s/15g36h/jkIUcWJD9XgwNt48AjRgkFJhgzagoum/rdTuV463NN1ewUXvS6lWNV3VUREnWRoB+q2TRO49HQZ7OWxgMeAJOya76zMu5QJUYERQS4YCkF0bdvdQDK7HhmAZOREaCFIRjWckAab/PvZd/jzwvfBlvnealHsEBrMYyzZp4cG8o6hl9Fslr1IA3IRAcJdOvwVkEjGRK8/D4b3irjLkjtnvMDjKygIwfkrpsP8gGf0kMVLVf76uXQfcb3oTXFzRgdCMnxPU1WvIcvtiHTyPT2oLsvfhayL/5QF8NbafaOKcWGH4kJbElBVlaPxRniB339I5h3tKHEHg3DZdWeuAmLlxEUdrbAwGYQbtHT1nGIM7hrTf0Kv8/xwJ4teAH/3KyN9MuScYXnQtfAeH/5Ab6OZ8Xtr8YHOwkEFg9COT4zpx47zn3CxAhnKXmgXIBlkRXTTHhewf7NQ24CCUMgoh68iybBggpB0RekLHsAFUR90LQ1a2mBn6fcWxQXc8005wqozrn61fv/n9jpmDS6SFI83f41Qj/xfySiCBCG06HQkF3WzgEHddtBiKANgnhvDhaKFL5F7svEambWSlppQAI5rvFTJoMd/i2QQEoTI2NF3UURMZ6lWlW7MlxFig9xTMBna4RhvrjCYX0EBcM7o3moKa1KHYJjOLMy+gK+EL0KGrv2YNmwkh7zYgS7KqFJNoX1y6mQB5MZw0c9JAndvm45Bj8t1w4yKFjV3IfVm6eAWE4QNjElXgT66RAtJwYynVk/TES5unLvGM9twIhkSuGwexoiU9BVTftgBbFmocZWAQ714Z0AXWikuN8uN7X5fMIqEhXg1BZdFCtUU9pxRhQFwHH2PFs6T9WMJFASR4nJGSYGSAnuk9C1FsVIPDE4p6VD/cHQLxoQBtUSJPEH8ufTncv00cZOUOQZr5r9ll1pSwVmCkMBFBcVgltYBVvRNNZYUxUBS/LhGVpog5QuhaGGXD0nXpxLiLULcvLKAM6lr9nULE5NGChduaHljJgXTddaMD/5XZBCUSp16Uuwo7FnlgCbAq0swlgSp8NDOb2ehNrS5kxZTIvpECni9rELghtbTHF6nBmUnxaqxFiPCC5PeGaEeCwmhNHP1/riELZBdr2eVA0JDHb9wFb77ghERI2WseFM5uMXDLUwgIjEkrDv8RiEBG5XbgQ9XIVvcWjhOdQm6mpVxkBEgcF8C2JP6pjEKBroO2WvRj/oFm2gpltMGpTmH7u7XIBWeYjQ8+gmXPu7I7wmRSprgfQuZtIr6qSPbf3J4j70m+dl2k86JiGFf7Kb4isjzsWiK/PgX+H1Nv2xr3WMAAAAASUVORK5CYII="),
            nfts,
            default_nfts,
            marketplace_actorid: None,
            main_contract: None
        }
    );
    
    assert!(!res.main_failed());
}

pub fn init_nft_with_marketplace(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);
    
    let nfts = nfts_tests();
    let default_nfts = default_nfts_tests();
    
    let res = nft.send(
        USERS[0],
        InitNFT {
            name: String::from("RutzoOnChainToken"),
            symbol: String::from("ROCT"),
            base_uri: String::from(""),
            royalties: None,
            base_image: String::from("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAAAYCAYAAAAMAljuAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAVhSURBVHgB7VrdUttGFD4rCaYXdHDewJ4AM70q3HVaIOIJoE+AeQLgtpgiF9Nb8BNgniDkCXAwzeQu3GUmkMGP4JnmohNJuz1HK8mrHxtJdlLT+JshslbnSKv99vwqAFNMFFgWIfPBKnFu7AoBe3JEXNq6W39bsbowxVjxKCHrHxtHPhGl6BXRBcGancXaGUwxNgwkZPXu2GSMvQSfCAHQdmxn5zv87eqGBRpsS0nRZaBb1wu/XcCEgSy7XbF68ISQIOTnDyfLuiZO8aepDPds3VkJXFScLB/oxpz9LG4M9S3ICOOTe9ZekYuq6qnjKfen+aFli9c3i4fWL3eNPQaiBDngOG5LN4ytvHoEjekPwQZd/XiyC5w/yypvqBfQPZ3iS/hxAnqMiwuhaZtoBeUZ13hYuz9uAWcMadwOZFC+yZiG52ILZbbW7xpn14u1/WEToMWCjHDmdDpYcb1/StCi58flV+8aRzhDyz99LV+Y7eKhDJkhum9/sKy1+5OcelL3s/15g36h/jkIUcWJD9XgwNt48AjRgkFJhgzagoum/rdTuV463NN1ewUXvS6lWNV3VUREnWRoB+q2TRO49HQZ7OWxgMeAJOya76zMu5QJUYERQS4YCkF0bdvdQDK7HhmAZOREaCFIRjWckAab/PvZd/jzwvfBlvnealHsEBrMYyzZp4cG8o6hl9Fslr1IA3IRAcJdOvwVkEjGRK8/D4b3irjLkjtnvMDjKygIwfkrpsP8gGf0kMVLVf76uXQfcb3oTXFzRgdCMnxPU1WvIcvtiHTyPT2oLsvfhayL/5QF8NbafaOKcWGH4kJbElBVlaPxRniB339I5h3tKHEHg3DZdWeuAmLlxEUdrbAwGYQbtHT1nGIM7hrTf0Kv8/xwJ4teAH/3KyN9MuScYXnQtfAeH/5Ab6OZ8Xtr8YHOwkEFg9COT4zpx47zn3CxAhnKXmgXIBlkRXTTHhewf7NQ24CCUMgoh68iybBggpB0RekLHsAFUR90LQ1a2mBn6fcWxQXc8005wqozrn61fv/n9jpmDS6SFI83f41Qj/xfySiCBCG06HQkF3WzgEHddtBiKANgnhvDhaKFL5F7svEambWSlppQAI5rvFTJoMd/i2QQEoTI2NF3UURMZ6lWlW7MlxFig9xTMBna4RhvrjCYX0EBcM7o3moKa1KHYJjOLMy+gK+EL0KGrv2YNmwkh7zYgS7KqFJNoX1y6mQB5MZw0c9JAndvm45Bj8t1w4yKFjV3IfVm6eAWE4QNjElXgT66RAtJwYynVk/TES5unLvGM9twIhkSuGwexoiU9BVTftgBbFmocZWAQ714Z0AXWikuN8uN7X5fMIqEhXg1BZdFCtUU9pxRhQFwHH2PFs6T9WMJFASR4nJGSYGSAnuk9C1FsVIPDE4p6VD/cHQLxoQBtUSJPEH8ufTncv00cZOUOQZr5r9ll1pSwVmCkMBFBcVgltYBVvRNNZYUxUBS/LhGVpog5QuhaGGXD0nXpxLiLULcvLKAM6lr9nULE5NGChduaHljJgXTddaMD/5XZBCUSp16Uuwo7FnlgCbAq0swlgSp8NDOb2ehNrS5kxZTIvpECni9rELghtbTHF6nBmUnxaqxFiPCC5PeGaEeCwmhNHP1/riELZBdr2eVA0JDHb9wFb77ghERI2WseFM5uMXDLUwgIjEkrDv8RiEBG5XbgQ9XIVvcWjhOdQm6mpVxkBEgcF8C2JP6pjEKBroO2WvRj/oFm2gpltMGpTmH7u7XIBWeYjQ8+gmXPu7I7wmRSprgfQuZtIr6qSPbf3J4j70m+dl2k86JiGFf7Kb4isjzsWiK/PgX+H1Nv2xr3WMAAAAASUVORK5CYII="),
            nfts,
            default_nfts,
            marketplace_actorid: Some(MARKETPLACE_ID.into()),
            main_contract: Some(MAINCONTRACT_ID.into())
        }
    );
    
    assert!(!res.main_failed());
}

pub fn approve_marketplace(nft: &Program<'_>, owner: u64, transaction_id: u64, marketplace_id: u64) -> RunResult {
    nft.send(
        owner,
        NFTAction::ApproveMarketplace {
            transaction_id,
            marketplace_id: marketplace_id.into()
        }
    )
}

pub fn mint_default(nft: &Program<'_>, owner: u64, transaction_id: u64, to: u64) -> RunResult {
    nft.send(
        owner,
        NFTAction::MintDefault {
            transaction_id,
            to: to.into()
        }
    )
}

pub fn mint(nft: &Program<'_>, owner: u64, transaction_id: u64, token_metadata: TokenMetadata, nft_data: RutzoNft) -> RunResult {
    nft.send(
        owner,
        NFTAction::Mint {
            transaction_id,
            nft_data,
            token_metadata
        }
    )
}

pub fn burn(nft: &Program<'_>, owner: u64, token_id: u64, transaction_id: u64) -> RunResult {
    nft.send(
        owner,
        NFTAction::Burn {
            transaction_id,
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(nft: &Program<'_>, approved: u64, to: u64, token_id: u64, transaction_id: u64) -> RunResult {
    nft.send(
        approved,
        NFTAction::Transfer {
            transaction_id,
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program<'_>, approved: u64, to: u64, token_id: u64, transaction_id: u64) -> RunResult {
    nft.send(
        approved,
        NFTAction::Approve {
            transaction_id,
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn check_token_uri(
    nft: &Program<'_>,
    token_id: u64,
    metadata: TokenMetadata,
    content: RutzoNft,
) {
    match nft.read_state_using_wasm::<TokenId, Option<Vec<u8>>>(
        "token_uri2",
        WASM_BINARY.into(),
        Some(token_id.into()),
    ) {
        Ok(token_uri) => {
            let token_uri = TokenURI::decode(&mut token_uri.unwrap().as_ref()).unwrap();

            let rec_metadata = token_uri.metadata;
            let rec_content = token_uri.content;

            // since they don't have PartialEq do it manually
            if metadata.name != rec_metadata.name {
                std::panic!("Metadata name is different");
            }
            if metadata.description != rec_metadata.description {
                std::panic!("Metadata description is different");
            }
            if metadata.media != rec_metadata.media {
                std::panic!("Metadata media is different");
            }
            if metadata.reference != rec_metadata.reference {
                std::panic!("Metadata reference is different");
            }
            if content.media != rec_content.media {
                std::panic!("Nft media is different");
            }
            if content.nft_type != rec_content.nft_type {
                std::panic!("Nft type is different");
            }
            if content.power != rec_content.power {
                std::panic!("Nft power is different");
            }
        }
        _ => unreachable!(
            "Unreachable metastate reply for the OnChainNFTQuery::TokenURI payload has occured"
        ),
    }
}

pub fn check_token_from_state(nft: &Program<'_>, owner_id: u64, token_id: u64) {
    match nft.read_state_using_wasm::<NFTQuery, Option<Vec<u8>>>(
        "base",
        WASM_BINARY.into(),
        Some(NFTQuery::Token {
            token_id: token_id.into(),
        }),
    ) {
        Ok(reply) => {
            let NFTQueryReply::Token { token } = NFTQueryReply::decode(&mut reply.unwrap().as_ref()).unwrap() else {
                std::panic!()
            };

            let true_token_id = token.id;
            let true_owner_id = token.owner_id;

            if !(ActorId::from(owner_id) == true_owner_id
                && TokenId::from(token_id) == true_token_id)
            {
                std::panic!(
                    "There is no such token with token_id ({token_id:?}) for the owner ({owner_id:?})"
                )
            }
        }
        _ => {
            unreachable!("Unreachable metastate reply for the NFTQuery::Token payload has occured")
        }
    }
}


pub fn check_token_from_state_bool(nft: &Program<'_>, owner_id: u64, token_id: u64) -> bool {
    match nft.read_state_using_wasm::<NFTQuery, Option<Vec<u8>>>(
        "base",
        WASM_BINARY.into(),
        Some(NFTQuery::Token {
            token_id: token_id.into(),
        }),
    ) {
        Ok(reply) => {
            let NFTQueryReply::Token { token } = NFTQueryReply::decode(&mut reply.unwrap().as_ref()).unwrap() else {
                std::panic!()
            };

            let true_token_id = token.id;
            let true_owner_id = token.owner_id;

            if !(ActorId::from(owner_id) == true_owner_id
                && TokenId::from(token_id) == true_token_id)
            {
                return false;
            }
            true
        }
        _ => {
            unreachable!("Unreachable metastate reply for the NFTQuery::Token payload has occured")
        }
    }
}


pub fn one_nft() -> (TokenMetadata, RutzoNft) {
    let metadata = TokenMetadata {
        name: String::from("Rutzo"),
        description: String::from("Description"),
        media: String::from("http://"),
        reference: String::from("http://")
    };
    let nft = RutzoNft {
        media: String::from("data:image/png;base64,asdfj3r2ec-//+++KLJFDSA3r"),
        nft_type: String::from("sand"),
        power: 5
    };
    (metadata, nft)
}

pub fn nfts_tests() -> Vec<(TokenMetadata, RutzoNft)> {
    let mut nfts = vec![];
    nfts.push((
        TokenMetadata {
            name: String::from("CriptoKitty"),
            description: String::from("Description"),
            media: String::from("http://"),
            reference: String::from("http://")
        },
        RutzoNft {
            media: String::from("data:image/png;base64,asdfjKLJFDSA3r"),
            nft_type: String::from("fire"),
            power: 3
        }
    ));
    nfts.push((
        TokenMetadata {
            name: String::from("DomCat"),
            description: String::from("Description"),
            media: String::from("http://"),
            reference: String::from("http://")
        },
        RutzoNft {
            media: String::from("data:image/png;base64,LKFDSA324LK2JF23"),
            nft_type: String::from("ice"),
            power: 2
        }
    ));
    nfts.push((
        TokenMetadata {
            name: String::from("Tom"),
            description: String::from("Description"),
            media: String::from("http://"),
            reference: String::from("http://")
        },
        RutzoNft {
            media: String::from("data:image/png;base64,fs0823lsfFSDF213"),
            nft_type: String::from("water"),
            power: 2
        }
    ));
    
    nfts
}

pub fn default_nfts_tests() -> Vec<(TokenMetadata, RutzoNft)> {
    let mut default_nfts = vec![];
    default_nfts.push((
        TokenMetadata {
            name: String::from("Jose"),
            description: String::from("Description"),
            media: String::from("http://"),
            reference: String::from("hyyp://")
        },
        RutzoNft {
            media: String::from("data:image/png;base64,j4k23LNK433"),
            nft_type: String::from("ice"),
            power: 1
        }
    ));
    default_nfts.push((
        TokenMetadata {
            name: String::from("Marvel"),
            description: String::from("Description"),
            media: String::from("http://"),
            reference: String::from("http://")
        },
        RutzoNft {
            media: String::from("data:image/png;base64,J423Lkjr23232134"),
            nft_type: String::from("water"),
            power: 2
        }
    ));
    
    default_nfts
}