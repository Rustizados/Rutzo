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



/*
TEXTO PARA SUBIR EN VARA:

DEFAULT NFTS:

[
    [
        {
            "name": "Jose",
            "description": "Description",
            "media": "https://",
            "reference": "https://"
        },
        {
            "media": "ImageBASE64-1",
            "nft_type": "water",
            "power": "3"
        }
    ],
    [
        {
            "name": "Rutzo",
            "description": "Description",
            "media": "https://",
            "reference": "https://"
        },
        {
            "media": "ImageBASE64-2",
            "nft_type": "ice",
            "power": "1"
        }
    ]
]





NFTS IN SALE


[
    [
        {
            "name": "Rusty Robot",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer sed pharetra lacus. Phasellus rhoncus quam odio, non laoreet risus tincidunt vel. ",
            "media": "https://",
            "reference": "https://"
        },
        {
            "media": "PGltZyBhbHQ9IiIgc3JjPSJkYXRhOmltYWdlL3BuZztiYXNlNjQsaVZCT1J3MEtHZ29BQUFBTlNVaEVVZ0FBQU1NQUFBRVRDQU1BQUFDODZFSURBQUFBQkdkQlRVRUFBTEdQQy94aEJRQUFBQUZ6VWtkQ0FLN09IT2tBQUFEU1VFeFVSVWR3VFAvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vL3hjUk9uaEI5di8vLy9IR1JNVFV1MU5lVVhoQjhXdCtaVHhIT1hnLy9EUXdVbmgrZVd0dGdWOXFZWmVoa1NVZVNINnhkRWRRUTE2RFYyZVpYRjlrZGtkT1Y0VEdkblZENm9XUWdWSjJTK0xpNWk4ZFowWm1QekEyTWxCUGE2aXF0NG1KbTZlMW9YZDZsRzlEM01yVHd2SHc4M1prTzJSQndzTEJ6RUU3WWtJcWgxbytxTFhFcnR1MVE5VFgxcGlacUZSRmpCOFlRSlorUDVuZmlMR2RSN3Y0cXZQMzZSRVZFTi9LYmljb2tvb0FBQUFOZEZKT1V3RHBqMEl3Q2lvRElCQzlwbVQ2VmpSbEFBQVRiMGxFUVZSNDJ1eWFEMitidkJiR2w3WnJkOXVGQ1dPSVk0UDVFMHdUU0NoTU5JQlNxbXJwOS85TTd6a21iZGMxdS9mcVhiWVNhVWRSVkttSjVGK2U4NXh6YlBQaHc3NjRHR2g4dVBqdy84VEZ4Y2VQSC84ejBJQ2xmYnk0K0Y4QUY3aitUOE1ORGZKZk1GQUNCRGcvUHh0b25KK2ZJd1pRL0J3QkFjNCtuMXlPUmwrR0dLUFI1Y25uTThUNGlSU1lSWi9PVDA5R3QzZmh4QjBQTWR4MWVMY1pYWjcyRlB0Rk9EKzkrcnFhakljZGsrVTFVdXlCQUNkOE9qc1pyZHp4OEdPeUdwMmN2OGtucmNMWjFmVjZmQnd4dWI0NisxRUpSRGk5WExyall3bjNkblQ2R2tLck1GcU9qeW1XbDYrVXVMZ0FPMS9kam84cmxsZmdpUmVHajUvT1Q2N2RJMk53cjhIWVQwSm9NNHdtNDJPTDlYZVd3RXk2WEkyUEwxWlh6MEtnRE5lVEkyUnd2NTd1SElGdXVGeU9qekZXSitkOU1sMWdYWjBjSmNOa3RDdE53UEI1TXo3T3VQMnNHYUFxblovY0hTbkRIU1lUV2hyc0VCNHBRM2lwS3hOYWVyUStVZ1kwQkRKQWR4aTVSOHJnOXFiR3N2UmxmS3p4NWV3dncxK0d2d3gvR2Y0eUhCL0QvYnl1NTNWVHo0K1dZZDdFUlRlTjFwVmhGSTE3akF4dTVWaEZ4M0lsMTFWc09NYnZvZmlkRFBkMTdGaFZ4MXF6NVdUU3hJWmxHUFg5VVRIVWhXRUF3cW8xUzFNbFpBMFFodUVVOWZFd3pDdkRjb3dxVytVbWhPQSt5WnJZc2d6SEtkempZTGh2TE1leXRCZE1IWndtdkt0aWtBSTREbXlMMzhJQVJqQndzVVV6RmExR0tDbWx2c29LdzRsamtDZXVoODR3QnlNNGptSEVWYWEwREtWWkNtUmdIWmc4dGc1dGk4TXpnQkYyRVhlckp4bk1rbEFhaUxBQ1N6aVcvbS9sRHBYaHZ1blhieFZ4M0lRN0JJaHJZS0NDMVUwVE8zRVA0UnpLRmdkbTBFYkFLSnFpNkpSQUNYUnNLREtvS0lXTy9mUVo0MEFKZFZBR05BSlVUM2dWelNyTFFyRnpBNzZCSWFnbmVHcXZDd2ZzN3VEYmdTZ095T0JxSTFqNGlwdTdWdGxNNUUrcFpKWTUxd3kyblVIWnhVd3krb3c2UUVJZGpPRys3dk5ELzhoRkoweEIySXNkUUF4dGFrcFNVbGU2OEQ0bGxGTzdBMkY0TmdJVWY4dW9Rcy9NSWY5ZkdIUmhvb0ltdHN6d3MxYXMxVEFPVVdjUHcvQlNUNUVCVWtrRlpxdjQ5d3liWGdkL2tmSU1LK3dMQTVwNy90NE03Z3RCbnlYRmpRaHlNeWV2R0RpYW1tSXljWndHWHpIOG1pMStuY0Y5VHFOZFo0UDJ0VEtWTWx2Rzh2S0ZRVThiOEpZdStMeHlvRisvK2hiWTR2MFlzSjYrQ2pCcnN3U0cxc3k1TUY4eEJOZ2tiRHVCMGdRSUw3YmUyY0o5SjRiNkJ3THNEbFlIdXg0cXpGd1JsYmZmTWVoWTJITGE2WkhEK3ZHNzlic3d2RUV3Y05ZREJsUFFWcWs4VjB4WHA3SjhaaUNwaEFIV2VRUHc3eUYramNIZHR4Q2ptTUtxVzBwM1hWb3dsWmNsVHQvVUo5Sk9VeW1peWpGZWU3cFB3M2orNXhtS2ZRaFdkZE9DbDlYT0RMQjh3WlhPSldnT2hCQTdWYnlKNHowTWdILy9weG5xdlRJWWpkYmh5YzlsYVhwQklFQ0hKQ0FMWkxCdHRxNzI0dis3NnZSTERQdldnV1dKUTAzTm4rcHFXZWIrZzVjb0tiMXZGQkFnbld5U3Jhczluc2JLZlA5bkdmYks0TUMrZ1BsUW1JQmhSNUVuczBEa2dqN01GajJEcEdIV0dNYWhiUDByRE5YZWRMRGloZ1dlS1Rhdzhla2hXbUJRd09CNXRtWkliY3F6MnRxZmlOV2ZaWWozTDZMcDdnTEs4cTBvTnpzaGZFZ2x3VWdRUERGSUh0WDd2MjNFZjVSaHZuOE5UVFpSSXFmYmpkaHV5aDdDOHhPbWdFR25FaUVwR09LbkRJWTdBSWFPY3h1YTI3Yk10M21ldDNqSzF5YWNNY1lYSUlPVUVvcXJiY3M2M3U4SFl6NEVCdXhrck4yVzVaWUpCU0hhbGhIT3ViU0o1RXdJWVV1WnluWDhFME84UDRNRk9sREtaYjdkYkVFSGdhRmc0WXdDaEZTS0VoblowS3lIeklDNTVFTkw1bCt2dHlDRldiYTVvcElDREdNeUpiSXZydURxdWhpMERqNzFLUmZiN1ZmdGFBSGJhcVlad00za0dCaWdMbWtHU2szUUFmb2JVMjFaS3RFem9DVndZRXExcHdmTFlEU1paa2dDYjlzVDRPRVNPSnR4cm51MGxnRVpuT0V5VkJuUlkvWkRnQnVJM2VHU2d2NkFERGp4U2RnRzJYWmRESmVoYUdDUmdPRFBBdnE4RllYU3hIWU0wQ0hVQXJKcFVnMDNsNHBhYWdodUw5cW5vZThWQXlpUlF1Q3QwRkRyVXVYQ0lDRTVKUWtGR0FXSjFFSjV4UU1OelJEQjhnSENSb2JCNmxDNUpBcWhJOWcrSlkrUGozbk9LR05nYWFoTEhJcFMrdmlZeWpSTDdhd3luTUV5ckNWTUZqUkZZOE9DT2NPOU5JYnUyQkt3WUE4VTJhUXJqQUgzYVp0UVFobDZBb1hnZ3VtQlExREdvVGpaanphMGE5c09oOXdmbkdvaUpadm1kQ2VFS2huNGdlVXcrcWtTY2dxNm5CMmxNcXVzQWZlNHVNc0lZWFFLY3FBUVlBTndORlA0anBMQStDcDFLZzJZd2JLS09nTkxNQ0lUbkZYTFZwUmxubS9nWFcyRUV0eE9JeW54Nkh2QWZyREExaEhsSzRVemFvdVRkd3QrYmhrTUd6aUV3N1FFeWRURjFvRDlZT0c5OUR5Q0JVdVNFSVdITkhjd3RlWkM5WHNKc0xURWkwVnJ5SDdBSnpLYU9iUTVmYWgzZXd1YmlCYmFuT0FjS05RMGpTWnpGNStoc1p4Qk00QVN0YnRlcjJIenpIQkxuZU9PdE4vUnlVbGR4WGpJTjJRRy9XaU1GUmRWVVZUMWVwSXFUS1MyTlZ1TmtOVlZnWXQzQnAxTFZxeFhaMms1aXFhZWhLelhRQ2dlZFZYY1gwNy9SSVBCNUpLMSs0bGhwVTdjek5lN2NUVWk0YVRxbi8zcDc2YUh1eGQxK2ljQUhJMWlGVjBJSFk5ejZCU2NSRmxUb0Q0TzNvb2F6c0FaOFBuRHVLcUt1SnBFbk4xTXFaemk4SnF0RzdRRHNHa0lqZk9tMXczRDAzcUZjVFVOdzY2ZVJCRzhvRzF6RG45TnVGcmo1VFErRUdIRjhhdm5CUWJGb0ZjWU54MEp2QUFHaXlpTElzNVpCR0duaWNkWGxUWThacFRoV0crdmc0YmdhVjFjNCtwbUpmd2tDQlkyTGo0alRMT2t2cytXTndWR3JEbjJ0SWtoK0FGL1phUG9oTXB6NGZ1ZWp4Q2M5RHJZUWFMRVhkZGxrR1lOSkpXenAwOE1nZ0hOME53d1BOUm9SZUFsQUVINlhBSVpzT05CKzA3OGdOMTArbEp1V0F5NjZFT0hodWlBUWQ5bDVjSERndXo4a0NhSjJPUmlDcmJ3Z3lDWXRUZE5NWkRhYXUwbWh4aWlhS2JRbGhXMUo3YmRQK21RQjdPRWh4U21XRW1vZ3FsRGdUSHNSUkxNWnJNSHIzdDcydmRlT2xoOTZ5cWFWZDdtWGhCNFVFakRrRkI4MEdIbStZdkE4N3paekF0TTJNYVJjTXB0MEdHR0VDdThxQjRBQS9hMHZoWk44d2RZR0t4MkZwQW9oUDdBOFpoTVlPSUFXUERnS1Q2RkNFTmd3RS9CUjIrcVloZ011dWtXelEwUzlFRTF3elNFTG4zYitrRnl0MXkyZ2lzdWV3YVphQjI4bVpkUGkvZk1KZmM3QmtTWTlyL3N3d09nMENnTGNiWHdra3dSZXJlOFhUS0daa0I1cGpzRy9IemV2S2NPVDNlNytEZ1ZKRklJS2UvNXMyOFFNNCtBSC9BSEQwRUp2TmFsU2pHYy9ZaTBrUzJTeVBEd0RTbHVpbmU4MjMyK1k3ZGlITElWR0RjSkhnRGhRWGRuY0VNRUkvY2l3ZkFUMzArU3hUL3RuUTlYb3pvV3dLMk9mWTZPU3hkaElFeGFtbXJMVW9KNFFCaW5vTWljNC9mL1N1L2VBSjNxV2xjdDBQWnQwbk1za2tMdkwvZFBiaWdKcWd0OUE4THBNM1NJMzcrQnhML2Q1bS9zUCt2WUN0Wnd5NEJoSnRUd2I5c2R1aFBQZzNUYlJsZkdnZ1FJQWErUmgzNGlGQUc0dzh2cnU2dlZLNjhkMyt2d1VCb1RKdE5YOTFQdzNOa1F4YkpubDhBQVJkUHNJUVpWL0RPMHNjeGMxeDNwTUJwaWM0eE1vTFRoNWErNzFkNzY1cnhiaHVWTko1QkhBd04wdm1EamoyTWdHWTcwa2E3cXM5bDRXRk9VQkxxcjYzTWNFWTEwaUxlWDZEcS9jR0IwVWF1aTgzdC9IbTVxYTdvaG9BVVEvdkZ5Q0g4dng3cGx6VWV1Tzd6RTNCVXh4b0pCZDIzYjFXSFR0SFFNVFkvZ083L3ZicTQyVXNPRzk1SDlwN2FtVzlOMmJSRXVSZEIwWjZwdDYrNFlrT3hLRXdKQ3R6VUFBZi9XSUVKaFh3MWUvY3lXYVBjTXBUWEJHT0FPR0VRS1ZQYlM2TDNRNHFVUmxiWTBYaUs0R215YWtJOWoxLzFvZzA5L3IyOWczOFk5aVNYRUJlb0JwUjVXdlRTYXpVd0RodEtiUzQvUVVBRm9UUmhtWFV0bGs5a1lITnVlM3Y2clp0ak92YUhRVzkrSUh4MVFEM2JkOTdvYUVrRU1LbDE2Sm00SnRYUjlCSjBGMkpHdFFwVUtBUlo0WnViOVRlWFBuNzdwdTRIN3ZYOSt2L2grUXpTd3BTcUJzSFhRQWJqNGRDd29RSHdOK3JYcFJOVVJkQWJSZGU1Qy9qcXpKdEFWMXI5cFhYMStCbU1qOTZ6ZmZiKzYxMXhYNUtmZ3ZibzZNbEVMUDhDT2dBd0lMQWJEQ2t1ZHVQanoranpLSTRpOXcwZUE4TzV2eTJ0K3QxdTlaMTNFcDVzN0V4bmdOWnZoUlQzSWpLSUlGR09QcDJBMUpoUmlNbFhWOFo1S3R5aHl5MU9oRmhMRFVnMjNXNTg3SUNpWWlzWTAweTNOdENhWlB6SHpSUVFKMG5pb1dTQy9lS2tUZlFicW11V0wzSnhBTW1oNTNpUzV2ZGg0eGw5amMybW9nWXB3VldqeGtla0hiTG9ZUk5vTTh6eU5tSUxCUkljQTQ5S2lwd0p5Y0JZRXdiMlIzRjFzUEpXM3lYbFpFRHRIRnFhcjVwUk1jOEVBSFp1RkRBZ1JRT0l0Y2o5M1VaZ3E4NExid0V1TW56czBwd25MZktRblhqTHhMTk82amhiRnlNYTgxVlpMTmJCZ01yZUF3YmJtaTF3RFBaQWc4SXdtcGxNM1BNZVB3dGdBUERWZ1BCb1VwbXYvZ1A1Wlk1cEpDUDVBcDFzdTJKS2xMZ3JCY0JjMHMvSlc0M010SjVEdnNZQUFRdzVlckZsK3hzMGN6QW02dWNpYWF1b2NSdFZGSGtINmZlODFOS2U5K1RtdkZCZ0k2aUVQZkdKbW5HZldVNDdaVTFTbzRNWWFNOVVpQWoxNGpjM0tiMlB1c2NHSWRZMEJOREN2QTU5NTZtSXhmSHdjRmdXTXBRT0dkbVdhWG9QVHdOdVp4MjZ3YSswcE43T01lOHpuUHhhREFzWkdSVEh4NE9WUFFUOUprMHNqdERRWG41SWZVeXVMT1NjOEk2d29HWEtNUkpNZ21Ick5MdTdRMnBvSVJ2WXJkamozQ1RoNHpUQUJMd2ltck9sMUFGdGNtOExqSVNpQ0Jaem5nMko0R1QxRk9sNGhhMzRsd3piWENLRit5bjl4bjZuUjB4UGtlY1ZvYmt5Uy9Wb2pCQ25BZlFrWnFWSHhCRnFBdExzRmd2Ylh6S0dRWW9ocmxLTzJDTHBaZjJtT043UWFENjJkdjZNMXBCN2FQTGxjQjBzeVNBYkpJQmtrZzJTUURKSkJNa2dHeVNBWkpNT3VNeGlpZk9vS2h6anlsVXZGaWRFdEE2OGU0aFo3SC80bXB6d3k1QzlFWmtwSU8yVndsTkJ4aERScklaano2Z005REtVKzlNWGpiNGlpdktvSUkrWnRNWWdUczFCWisxUXFYMUhXTUJCOER4UWxleGZEdXYxTk1hQ2c5SE1NNTdHUzdnWkRnQXhKS1A3SkhIeVNTaENIU3BvWkxBWnppV05PMDFSQStnNTd3Y0FGZzVFNWxXdUFySWtQeDVhMUpDNDNEUzdPRTlQMkdGTEZXYmE0MkpjcFNob3JDdmNyejZXSzRxOUt2dHd5eE1lVEVQd0NQc2NFZ3lPMmczTnhtaENxVXNxcTRKRzB3WkQ2dnArRlFxQVZCaXBFb0NTQkJnWWJnQkFhSzZGbzVOQllNbkE0bElmQ1JFQnErR2lLSVFrWlZyWjlpa3IyenluODlkNzNXTW9QTTFUdEhKeS9aUENlKzRNbjl0UjZFd3hsU1QxUlNjNnJON0t5SGFONno4dTNGdjBoSmlRSXk5Q3lha3RnUnlrWHY1blhQaDFENlBMK3lJRjZJSVFyb1YrNms5aVBEVTVXdGl0a0VURmE5bW12TlBaVkJockVTbW5UTlFPNlBWK3FvZmFIVEtobkJ4alEyR2tWblBETEswR1RlTVhUejJtbzhKV3VzR0tnb21kaFpVVlMyVksxSFFqZGlkRGx0QjliUGRGUmxkL3VDd1pXTm4wb3lMemFNVXJ6ZmhhWGZLeEdFaW9NME1DekxMZDk4YUZFbk5GNzJSazJIVnRqVkFGSW9rQjNBTzJOU1EvUHVQQVQzT3VFd0FSK1hnZldGUWFvZGdTb0F6MEJtaVRCSkFTMzhkZ1V6cGlXNlFoMU1NNEd6VFBFd2hGQW9oQWJtK0hYa0JpRnl6QmlDWDhWZTBWY1QxZlVnQ0t4cXY4VlFkZXBvNXNIRXVONVJLTWJJcW1NamFvbldXMkVCblB2WlNaZHVrQkM2MzNVVzVxdjJJc3RuejNMdmVzTit2enorSDk1eE1yQlpVM1NSdi93a1JLODJ5bDNkeHlYS2gwOW9MRkZCajgyOXA1QlhoT1FESkpCTWtnR3lTQVpKSU5ra0F5U1FUTDhZeGxHODdacU8yT1lEeFlQN2RSMnh4QU5CbEU3dGQweExBYURSVHUxblRHTUJsRDBObW83WVpoSFVaUXZVSTVGSGtVdjNYT1QydTRZaEFoL3lxSzUydTRZOHVkeVJNM1ZkdWdQMFp0aWJGTGJvVS8vRVdRUk5WdmJIY05EOUZaTGJsTGJZV3hkbW5YZWRHMTNERVV0UjlGMGJYZTJKRVFRY2ZLaDJkcnVHUFRTSWFQRnE5M3RKclhkTWN3WHVXaERtaS9temRiS01aQmtrQXlTUVRKSUJza2dHU1NEWkpBTWtrRXlTQWJKSUJra2cyU1FESkxoLzVPQkV1NkV5Z2RMeXNudU1GQS9qQVBqdzdPSUV4STcvbzR3R002blp3b1kzREYyZ1NGNTErU2pkU1VJayswekdKc0pBVTFnYkozQkNUWTA2Y0RaOWpyci9wcUpQOVI1dDJpeHYyV0dkVDVKblhmYmlMSEJ3Z2hOTUpCNGZjUjk5MGxpc2xXRzlkMlV4MFRmNTR0SnlWN3cxbnJlQWQ4cVE3bzJLSVZLTldWU1llY1p6ckwzMytoZ3RzcXcva2ljaHV3b21aRmtCa0g1cy9YTGN0QndkeG04YXVKcnFuaUc0YjB4blZqWktzUDZrQUlNckZwTnBGcUpZRmNaMXFjN3dKQlVlbkNVdDJOVWttNDNMZ1Z2K1VPSWl3d3c2b3M1M3VzakU5bHVYRnJmUDJCYzhrSU1TSXltU2dqREM3S2ovUU5kMnhzSHFDSEQ1eG1HSTVLVjcwMkgxb2J5cFhqanNSamZkcjVFRzhoYjVmaEJqdU9XUHNrL1BaNk9kMk04alk3dHhPVER6VW1OSUE3OWJ0ZFVlN3VmNE9sSEx5K0ZEaWNiUHhubEgzV2RyOStqZTRwQWV5WERNVEFZZThxUTlQb254MzhkSEFEREtkdFRCbllLREFmQThMVi9tTzBwUTNiWS8xb3luSHlMOTVRaC9ZYnVjQ0FDMDU0NmhBSHVJQmpRSVE3OXZXVGdwMkJLSmNQWGs2TndINk9yNFJ4VmFoRFI5V3dmRmVHZmxwRlZNSUFpZXNrZWVnT280Zmpnb0lib0h6cjdaazNVT2F5OW9mS0kvaG5mTTRiNHJBNUtTMnY2Y3JwZkVCeFRwVDlxcUYyQzc0ODVVWDU2OUJ5aGhPaWZPZnZTMVNVT0d0SnpoSU8vQk1SaHo5OEhDdXIzRHY4Ym9ZWTRPdjM4MkxpemtPcUhaMGV2SVpRUWdxS1haaXpaVGMrZ0JzdmkzaUVTdklaUXErS2szLzkyZU5ycktidFllcjNUdzI5ZitpZW9oRmNSQkFSU0FFYi95MjRXa093RUNZN1hFRlR4Q1N3S09YYTFnSFRIYTVXd3Fnd0FRWlNkS3lqWi93UlljdXhzZVUzY3Z3RnhieFZVN04vRjZRQUFBQUJKUlU1RXJrSmdnZz09Ij4=",
            "nft_type": "ice",
            "power": "3"
        }
    ],
     [
        {
            "name": "Astonishing",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer sed pharetra lacus. Phasellus rhoncus quam odio, non laoreet risus tincidunt vel. ",
            "media": "https://",
            "reference": "https://"
        },
        {
            "media": "PGltZyBhbHQ9IiIgc3JjPSJkYXRhOmltYWdlL3BuZztiYXNlNjQsaVZCT1J3MEtHZ29BQUFBTlNVaEVVZ0FBQU1NQUFBRVhDQU1BQUFBbmVRQVZBQUFBQkdkQlRVRUFBTEdQQy94aEJRQUFBQUZ6VWtkQ0FLN09IT2tBQUFEblVFeFVSVWR3VEE0T0RoMGRIZjcrL3JlM3R4RVJFZjM5L2JPenM5ZlgxOERBd0VKQ1FtTmpZK3ZxN241K2ZudDdlL1B6ODNsNWVYcDZldjM5L2Z2Nys5L2YzK3ZyNjVtWm1heXNyTkxTMHJ1N3U3bTV1ZjdHL3hjUk92N1F1TVhHLy8vLy8vK3Qyd01DQS8zbmdQSEdSRFV2VVlYOXdQK2sxU1VmUmRUVTFrbzdZQkVPRTFGTmJNWEV6bXBzaW9tSW5pRWJIKy9Gc0doWmRxaW10VE1zTUdaUlNrdzdQWmlXcm4xMmo5T3hvUE9venJpYmlhcDNsb0ZxVmJtNTZlS2N4WmwvZHJpMndxYW4wY3FNc04yMlFzM3htcFY1UUdlM2tQalhqejFyVTNxbTZycVpRUUFDQmtkd1RKQjQ4UEVBQUFCTmRGSk9Vd0FaS1M5RERDQURFeXMrVXZ4aVlydzBVNUZFcVc1dFpweUtpdi8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy84QTFjMGhEQUFBRDkxSlJFRlVlTnJzblZsdm96b1VnRWZxcUROWFFxclVhdDVJaUN2Zk9wZzlMQUdTa0VWWDB4bngvLy9QOVRHUVFDQTBDNEVnY1I3U2xFRGl6MmZ4OFRITHQyOVY4dU5CNWR1UGIrY0k3UHJ6Z1FWSXZnSmd1ejAvcnZ4OC9nS0RFOENlVDQ4cndBRVVKeEdBNE9ucDErdUxJRDJtQ0MrdnY0RGpCTVdQYjV6ZzdWVlNQUXNUOFJHRnlKWTNGVjdlRW9wcUpUeTl2UWc2Rmg5YjhJeFRsQ0VTaEZkSkorTGpDOWFGMTZlU1BTVUlMNElzOWtPdzhGTFdCRU40RTJaRTdJdVFqZkJXaEdCcWVINFNabUtmWkNhQUpvb0lMeHV4WHpJRGM5b3JBaEJlQmRJekJzSWQrMGZlR2JEWU41RWw1aEpwYk9LV3BJdjlFMzF2VGVBTmZWUURXTk5ieGdCcW1JbDlGUDMxS1ltdlBLN2lYakpnS1RVbXh2QnJLdlpUMUY4Snc4K2ZUNjllVHhtOFZ6N09jWGV3ZXNwZ3ZUeHpSZng4L3Q2YlhLK2MrM0dIWUhyNExwR2VNaERwK3orYzRaL3ZrdGhYR1JnR2hvRmhZQmdZK3NsQVJVSklueGtvMXJlcjhYaTg4aFRhVXdZRjJwL0sxcVk5Wk5CeUJLQUxqQ2pwRlFPVml3UUFZU0RINmhHRDRZM0xzbzFwRk9HZU1GQzhHbGNKUm92SlpJRjd3RURqK2JoYVBHUUgwY0lpRDg4UVY5cFJJakdpaUVsTUg1cUJJbnQ3RW1Gc285akFpcVpwQ25sVUJxSlFkTUlWMHREazdUOXRhc1dzWVFaclFaRlNoMURrc1I2UEFVY0xoSlR4QmRJSVJaTU1zY08wb0kwdmt3WU1xakVHb25uSVFNZ2FYeW9yOGlnTUxET2FzWWlwalM4WDc5WTQyd3lEdk9WNUJMb0dBY0x0YlFOM0l3eHAwK2Y2VlFoakRUazNwUjhOTU5BWWoyK1NMVEltazhtOE93WnNHY2k0allIbEg0eGhjblUrZXlzRHhDRnZlMTNUUFpxT0pSUUZBREd4T21INE9wU2VIclF0aW80WXJvUzRqWUY4U2FERnVEcUgzV0lVcHoyd2ltbUNNQWx3K3d5eHJXM3JzaVBQZ0R6YjBFckd0cFVwb3Q1aFRwRXlUQ0xTTm9NTVU0SFlQa1hoMlFqWjRacHh4SWJscmJMZFZ0dTVIY08yYklPQ3JJemhxdWgwRTBNVUxCeUVhSkhCUzRvWks0M05GT2phSEkzTTBLRW9qbU5xWXl4amJBQTRYWVlHU2tQeWlxSm96eENRZGhtZzl5eEV0NlZKam8xdHlucmFXSTR5OFpjdU55c1FhamdoUXd1ejVFbzdtTkoxYm4wTHc2NEtZV3lsTGVVTnpZbHAraUdJbjIxZUpzZHVLZHJsR0hidE1nU1RIWXE5aXZLTGYyaG9qWmcyV05QV3lIa0RHRk9yREhnU0dGVlpIa1ZuQUlBd2E5SVpnaHZrR1NhNFRRYVpXWkpkRVk0TTVKL0hNSEppSXo1R21NaXRNa1EwM3Q3RTRMTVFaUjBodE14Z3hYTFYwSHkyTFkxR0xuSW5rMDRaakdNMThERmJRKzY1Q0NNVDVZYUdMaGpFMkRpT1NIakZFaVFVbnMwd2N0ajBwMU9HNHpMTUtrYVVXZmo2ZklTUmo0eE8vWUhHZXFtb1RhbDdnUmFZMkZBSDc5S1dqc2MzSGJubTZESlpsN3k2WTRadGJGektVUGJxbG0xSnUzNW95SG0xMWFsUHkrVlZudkJTaHJDUXRiYk5RTXIxREEwdEwyVXdqU05qYW5tTUsrWGRiRlo1S1FNYnE3UElGRVZSNnd3dU9nNnVLMlNNTGplbUpESUZiTFpIbmFCbGhnWEM1V3BYemtxV2xOcmhPWkdKRDNNdVg2WnpXMllJVUx5cW1UdndScDB4YXR0OEhzZW1VOEFRNzlwbFlGMDNMd1ZYTXpmRDRZV1pMMGVNZFF3NTB5S2RiUzlhWmxpVTVrQTVCaWR0bEg5ZWRPMktJU2hsMy9RS0JwTlM1aEJSVXZlZ1Fjc01FK2Q0N1NxK3dwWkdCbmVJUkJHTFNkc00wVkdCYkp1UHJWd1JOQmVZL0hCcEc1U0pZVHRydnpSQzdDelhpU2F0TXpDdnRvcGpYSDRPdDdZTjk5RFUwS1U4OUdSaUxBK0ZKcXV6Y1pwSFJLYUkxV3FiVm83bGs2RTBaT1pPM1dYNCtmbisvdjRaL3Y1akhIUVVGdkx2MWhsQUVXeGV6U1p3OW53RjVZQnFEemFaWFJtTDZNTjhUK1JmSnIvamJQRHcrVnBXZHd5THBJSUtoV0JEVjA2VUEwdzIzTG5CeDBlUUl2d0hEUC9Da1dFeVV0TXVHU0lib2FYUGk5dFE1ajZoQm9aZ1J4OEhCbzd3R3hoczdoUDdkYUF1R0ZoUXQvM0R0REt1enJ5WEVETS9EZ3lKR3Y2Z3ZTSUs2WGZiREc3YWthbHZyazlXODFCVVlzaWxVM2ErOHQxNnJuSE8vSm1QRXp0ZytNZ3oySnpCNGJZV0x6cGpNTTZxSmZFa2d0dlN4MmMxZzRNNlk5aWxvN0lacnYzYStRSExxUjNPWUpZWTFoMHpPT25zR1RJang2eGxZUE9jbkVQa2ZOcnZtTUZPUTZtNWRHaDh1bENjTUZnNVl6b3dPS09PR1NqZGQ3NXZuSzdLSkFISWlJNk1DY2FIdGRreEE4NHhRTTN4ZEYwWW9hTVJnaXZpTUNLNm5jVldUSE9SMWErcGFDd1RSUVFsWTNJS00rcE9hbVI1UFpoMVZablEzaXVpWUV6WjhZVnh1dFUxUlRGZlhUWHJxMlArMmtGMmJwaExvK3M2OC9xZ283VmRzVERFSlN2U2psdFRrOXdkRFhPL1UrNUM3dDN5R2pzcDlYMVlZMUZ1M3BnS1hsMllBN1Y4cmdPaHh3SFZyYWtaczhoMWJFeC9rdDN6YzlGSWJKY0JLdDltc1puVXJGbDZzeitxalNtL250WDZ1VC9NSTl4aWtyMnVYVDRzNVgwOHNwbVVScmVvNGRaejRWQnVCUzQwYWxlbUsvU1FNT1JYVVRvNEYwN0VCako0d21ENnpqNzlPVFhTT2RVTURwcmY0TkMzTThBaUJLSzJhOE1rd1VJMVp6bVlOSW10Wm1GS2JmQVBvcHNRYmo5SGx5eGNmb0dQNFVTVHVyRGtwR0hwOC8wbzIxaG5wdFRaT2JwTTVzRnV0NHNDWGpFN09UZjFpMnI0N3pBK0dFbFVDdVpYWDBQUXhEbnJPQXVOZ1gweU1ObUZDVVJXMlhENXNNanJWTjJlc3c0VXFVWHZUcFhxdzhMOElmTUc2a01Ld25wZ2Q5UDlTWnE2aHNOS0tLeENzU2F2aG5XcE9NT1Z4dkx5WFhEalJVR05YVXVEZVlBTWpNbzZtWTlvVUJHVVRLYWYrUFpMZVJ1OHBnbmNJckFxaTMxVll3T293VFRpQnE1d2IvVGFNZ3ZPaTBieHV1b01uNUlwUVc1bHgySUQwdkExZnBSb1dsd2FKV0FZSzV1U2E3ck5YRFRhK0xXVzJsK05IdWNjWm1FeS9aNldXOWV1S0Q0a0E5elFrMEhZL2xjcHF4R0w0c015aUtLaUc0aXVDN2FVMTBQQ0lJb1B6Y0FvY0l4eXk0bHdmayt4cUtHUmgyY1FNVXZLa2VPZm1Fdi9idmJtRHZlN0p3SmNwT0ttMXcrc2svbFBFbHZEcG05OGRzZDdVOGlZd25wdTZQTVZ0Nnd3OCtrMC9UdDN2VWNJVVRRQ3ErcUc2OWpKN01IOFhQYnJIaUhwYUlFcFB6dUFnbGN2N25JN3lYdmZiNFpSL1AycmF3cExzWVBGbmU0QTJNSzlpMlIrRndjcTN1MzJSY005cEFhR2dXRmdHQmdHaG9GaFlCZ1lCb2FCWVdBWUdBYUdnZUUrRFBMRkZSV2xlRlBOM0JjUVRmbmlXSHlRNWhnMDZkTEhLaWlTbEg4Z0JwYWsvUmVva2xUZnRIbnVTWEhUeGhnRVNTbzk3UVdyS3FsbG1CZi8zWGUrSjMzeEhLVThnOUFVQTFPRFZIbzJoSkxyMmlwN2tNVVRET0padHd1ZFNWS2ovaUJJbWlycFpRYmxFdE5TTGpQR2hobFlBN0NXR1lDaUN0SjBodkZteW95Vm01TUdXL2lwRnJvd1p4OExNekIzZGNwZWlTNUlncW9sRE96OUZEckNFang0K3NlTWVHd0RYMXVYMlRlb21CMTlQd2FWT1FNUkVrVm96RVRaTHdwS2FxK0txSVBWU2h4UmxWUytVZVZPekpvK2xZVHBGRHlEN1Q3bEg3R05PclJ1emcvaVh5REtjRGpiRTQ2N0V3UG1QNlFuN3FWeTU5WTBGaUVsQ1o0WnlmNTRCSnFrODRqRDNudmdLSnhCNFJFSXp6RzhFeFRXK2RCTXpzQmVXS2pUZVBTQ0xoR0pKOTJSWWNZREhPRjl5SDdQSzVyNEptR2J3UitWdjVkaFQ4NGc3Nk9va2h6Tjk4b1lGUDUxRzlnTGpJZ0k5Mk5ncmZFVUpzbFBnQTJvbm5KZ21FcWIxTVl3MkpLWW1oRi9ZYzJTcGhzTjczY3VNQkN1Vm5YdjcrcjlHR2FIWUEwL0JSN01MVURKZW5KMmtrSEVNL0FDWmpSVkRPS0J3Ym92QTRISXdtV3ZhMW1IQmgvYmtpU1dHYmdaZ2FicUdFZ3lwT1A3K1lPMnQrazV0RVBCaWNYTDhPSWxuODhoVklJNlNyWmtKYTFSYXhrZ0ZPZ3lRNjFpa0lVTlNWNnN1b2RWMWpNSSt5eURhY1JqZmFiT1pnTHZlNGlJTE5TekZxaFR0Z1dYR1ZnczI4dzJhWVE2ellCNWxCVXFmWG9HQjI3Z1JhM0xDNzVpd0ljOFJoVTFIdVpWMklhblBON0RVSlZ1MlhETk1GWUZqcFBoY2JMUXVEbVBhcGdyYlpxK0tJa0pKa2RnWGQzb3BPZ1A4eVRkbTBQblpDL2tXbi9JSGNnZklVeGtPZHRFMG5mN0xRa3V3WWZqc0lMelg1TjhBOGx0eVBkVzRkRnY2Zk9LRHp1VFI1NEQ2Vk5Od2NyczRwVHFrUml5WFB1R1oycDJQeGRWUEJhNlo0cllaNGFoSmpBd0RBd0R3OEF3TUF3TUE4UEFNREFNRFArM2MzWS9pdXBRQUhjbU0zSHZXQUVUMTkyOU1lSEJKb1dBVFZxZDlzVUhFdUxML3Y5LzBQWWNRSFN1T0FNaUgzdDdNS0ljby8zMWZKUWlwNVpoZkF6djc4MjFJUjBFdzI2VHNxWmFHc1JzQ0F6N3plYlFWQ3VDUUErQklkMXMwcWJhSkFqaUFUQ1lqdDVzM3B0cG84Qkl2eld2dS8zK2NFeWhsZW54c04vdjZtaXBFRm9ud0pCb0xhTGVhc0N4Z2FXa2RiUnhjQ0Z4WHd6SHkxWWU2bWoxSllQdUxSNE81NDA4MXROZVFHeDdqT2w5NlNyN3VscFJPcExvTlM4ZEtsemxLOXFUSlVTL3VmVlk2U3VmYS9XOXdkQVN3KytpbGIvcmEyWEJJSHRsWU5oQXpLS3N0aGFiajBNRTY1UGhQUXZYZlhwMU1MNnREVTB3bTN3a2tudUc2allZMGlQMjRlNlk3dXBxYWNKUlMzbE1lNDBITzQrekRKYkJNbGdHeTJBWkxJTmxzQXlXd1RKWUJzdGdHU3lEWmJBTWxzRXk5TTBROFpqNDdRaUpPZTJCSVlxSml0cGFyNDVGa2pTNmNud1h3MldSWnh1eUpieGJCaG0wdjJRZ2JmQ0gwQjBNS25sSWhDYXFPNFl0ZWN5aWdheTJnelptb09RaGEwOUNvcWpiT1kwWnRqWDlWbi85Uml2Sk8ySWdOYXVRNHE4WGowV2tHd1phK1RzQzJrcng1Z3NzUE5aYzBEWGJCcjZDdDh5OEJiMW1URmRDMVhUVHBneWlNaWtSY0djSnRiQ2hMN1BpWlVJRmpzU0pTUVJRZU0vVzNGZkVyL1JHdWUyRWdmTmJUcE9WcFV0ZnJMV21hK1VycUM4VmxGR28vWXg4RGxXdlNsUUdpTkk5TTBTbStiQW1Cek1Xd1Z4SjRVZ1dEOXJubE5LQUdJYWswWmUzeXFDclJ5TGpUREV4blIyQnMyaVNyL1dBRExKWWc0WGZySzNzaU9GR2FqVytibHBQQW9uMTdzWjV3aE9ES3Rad3VjMlFpSjd6MHByNVVEN05zZmNGUFBFc09KUTVNOFZxOFVoOHd0QlJYcm8xUHNSWm5UZXNLUURCSGVENkh4R3VnY094WkQyNXpkRFYrTERtMWM1RUpkd0RvN0dWbEV2T2NFZ0lGYVJNcWlUdTVJMnVycGxhbXpNd2NrKzVjS3RuWXMzUFcvWER6bHZyM21ENS81NC8vQlh6T0poUGIxdEcwRjNQcDJGWk1pS2p0c0tDUm9yRVRSTEZ2ZGVYYUt2WGw1cWx1ci9zT2g4YktRSXJHUjQyeDMrMGhPVEU0SXFSTWdnM1k1aCtlMW1xa1RLbzVjcy8wK2tFRFBFOUhpbEQ4TjM3QmdnWTFPTU1DRXBla0FFRFlzbEh5U0N6Y0RBTXI5N0xEOEpHYVlZZm52ZWFNVHg1QzNlTWh1RHV3bnZLR0l3aHZKVWZqdGdNdVNHV28vTW1SbjRXWmdBR1k0aUZLMGZHa0JoUDhsNXpoc3dRODVGQlNMSW96WUNHTUJBcldKUndOSTRrM1JVZ25CakFFTTl2eGhLak9mY0xpVHVmZWMrbEdUQTFHWWlaODVQd01WQXc3aS9uc3pkQUtCblFtd3pFWXVVU09YUUt5b254STBRNE0wTUpNWE1NUmFCRU9NeklZRlNvMkYrdW5Oa1ZoQndDS1p4L2wyNWI4K1MyaGJqTFh3NFNYRUV3WWlDTUtkNFF3M0htYy9NQXlYY25jU3IyNW9YalhQMWtkcnpVWFg3c2d5NS9jczYvTHp1WXk4d1FHQ05jUlVCTGdDa1FZNmdDclFNakdJVEpWY2twRUdPdzhwd1JUS2NWREFiQ1VBREdZT1VKQXFIS0NHY1lDREpNZVFXQ2FaVVJDb2FjWXJBeStZVGdJazFOaTYxNEtsNmVkdWZiZng4ZnRLZnRpbWJ5NFdlcU51emx5VERsem5iOUFWcXVzeG5kckJmN0FBQUFBRWxGVGtTdVFtQ0MiPg==",
            "nft_type": "water",
            "power": "2"
        }
    ],
     [
        {
            "name": "Cosmic Inferno",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer sed pharetra lacus. Phasellus rhoncus quam odio, non laoreet risus tincidunt vel. ",
            "media": "https://",
            "reference": "https://"
        },
        {
            "media": "PGltZyBhbHQ9IiIgc3JjPSJkYXRhOmltYWdlL3BuZztiYXNlNjQsaVZCT1J3MEtHZ29BQUFBTlNVaEVVZ0FBQU1NQUFBRVRDQU1BQUFDODZFSURBQUFBQkdkQlRVRUFBTEdQQy94aEJRQUFBQUZ6VWtkQ0FLN09IT2tBQUFIRlVFeFVSVWR3VFAvLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vLy8vL3hjUk92Ly8vKzZOSmUySEpmSEdST3lLSlBDUUtQQ2lMTzZTSS9DZkt2Q1RLdkNiS2V5QkplNllKKzJXSmZLbk1Fa0h4alF1VWdvR0NzWER6eVVmUjB3eE5mR3RNNVpaOVI0UkRPTGk1L0h4ODZ1cHVsSkxhK1o3SnUrZUttMXJnL1hhUFVJL1hqWW5FWXVJbXlNYk5VV3B6NFZzUGpFak9iclVPUmtjR2VyYU9oSUhKTkIxSjA1QkdKUmNKdExVMjI2cFJHbytMY3JYTkphbU9OM1pOMHVVejExZGRDaU1zRmFFemtNTXJIOU9KN0pyS09tVUxHaDF6aVo0b2E1dTh0T3pRSm1YcUdKVFBEb3dPK09iTW54NmtOdWlPaUJPVTdhNk5HZHBJcXE2YnpnSmtsYzNsb0hRaHg1cGUyOVp6RFdjd0pmSGY3NnlXczZtUjFzZnlvSURGVXdOR2VPOFFyYWJPMjB1MjZiUlZTK3V3bjUvS2Vpa05NeUtMVzFDcm9aSzVzWUhFMTE3UE8vOS9DL0oxQ0VIVUlsVjBsalV1QjR2S2RqNitrd3BoWFBNcEszWHI0dUluWjZITnp0eGFlVXFHamxVTlRBalhJelkyMFVvYlN3SWI5TGt3M2ExYVZhamcrRHo0S3ZtNGRiRGVyN3U3ZDdPUHc1N3FBSUFBQUFaZEZKT1V3QUJZd2tTRHpNQ1lqRWFXQ1JLODkzRWk2MjNhRzZpZG5OZkhKTUpBQUFYc0VsRVFWUjQydXlZNjA4YTJ4ckdkMUh4ZnVsbFh5Z1U1TzZ3OXVnc3QwcnNsQnkwdzdBUFlwb01rR0RUcG9GQTBnOUZLd2tDbmdRK2tCeGpESDcwN3ozdnU5WWFvSmZzbmMzZ2hlYThFSlFCY2YzbWVaNzNYY05QUDMxVGo2Q21IMmJCeW43Nis0TGxUMHhNVEUwK3lKcWFnclVCeVYrdUh3RW01eFlXWnFIc0Q2OW1aeGNXNWlZUjQ2OGttSnBibUxVdnpzL1B6TXpZeEIxdmcyV3o4Y1A0YURNUHNmZll4Ti9NbUUvTnZ4aDhnODFtRy93ODg5TnQ1Z2ZOMkw3OGQ0TWZORE0vdjJpZlhaaWJtcGorcnFrRUFRQThmcks4dFBUOElkYlM4dkxUeDREQktiNkxNQWtFUHo5ZCt2M3RtNGpzZUlnbHYzeno5cytsbFYrQll2SmJDR2hFRTNPemk4OVcvbmdWY1R6c2lteHRydnkyT0RzMzlVMHF3RWYyK1NkTHIyVEh3Ni9JcTZWZjV1M29wMEdDNmVtcEJmdjh5dVpMeDNoVVpITmwzcjR3TlRHUWJFQ1lzejliM3BJZDQxTHkxdkl6VUtLZkNZZ3pHR2w1eXpGT3RiVU1kdW9IKzlIRTVPejh5bmdoQU1USy9PemtoTWtBWVZqOFpWTWVNd1o1ODhuaXdwU0lOVHJwdDZXSVk5enE1ZExQNEtaSFBBMGd3OHAvSGVOWHIxWlFpRWNzRFhQMlh6Y2pZOGdnLzJFS01UMEJNbXc1eHJGZVBWMWM0QXpRbEpZaVk4a1FXWUxXTk0ydDlQaFB4M2pXNzQvdGM1eGg4ZW5iTVdWNCsyUVJBL0VJNHJEOFprd1ozaXl6UUVCbm5WOStPYVlNRUFnY2M5TlRFR2w1VEJsa0htcG9TelBQSGVOYXoyZit6L0F3R096anoyRGpEUFlmUUlleFpyRDlPSm4rRVhTNFBZYlllaW9KbFZvZjJ6eXNaeEk3WmlYajQ5aGI0OW1kSUx2aC9kWW9iblBHeFRKOUF2T1dpWTFWWDBvbDJPS0Q0a0g4bWtqZFhxWnRJMlpZeitLaWcwSHgyUHU1RTB6RVI4NXdLM21JSlh0ci83cmcrSWhqY1N2ekFZTmdsaXZvY3JuNEkvNFFISm1IbnVuMVJILzEzeXQ4Y1pTeEdEM0R1Mnp3eThWbmQ5d3Vwd3Z2N2tHTWJIeDBES1BOQXdSaEFNSGpjanBUUkFVSXR4UHViamVqTVNsR0ZZc1J6K25VamdrQTYzVkNlUktrbzJXYzdvU3pWeDZuRzErRjk0eklVQ1AxRWdSQmhJQUQ0SHFUV3JXamVwSkpyd2VlZWVDQUUvVFl5U1M0cjBaQzBkUEIrbnlJWnhrQXJwNGh3SUk5SHE5NlhXM1FPbFZ6ZFE4clBGNm5SSFh6ZEl6Q1VDUFRJWllSQ3JpYzNFUjh4VG9wVlp0RUpZUm1QS0s4YnJWVnBzNWVNakx4RVRGWXpVTnF4NHlCMDlNdmY1WjBXaVhqU3R2V1NNcmp4L0o0RXJvbWZhUTVWTXVGc2JEY1owZlRsMkJqd1FoUUFuUDVVTjV3UnFzMDJvWWlsMlBiTkpkakRIV3FvYjB3SE14MWFLakV1L3RtaUpzRXJwNkoyQ24zaCtxazFaS3ExWXBXanJlMXBJNFFPVlVydFl3eVNYcTluSUZCV0lxRmRTL2h4c0lsc293QWZnNFF5c0V0bzBrbDQ3cWthWVMwRzRSa0FvR1Fxam1NYXFWeHBYdTlwaEtNSXpqODlzUHlkelB2RW55bTRUekFMREMvNFBrTytVTVpDbFlxTitXeTBpV05acGVRWkxST1NpVkRVcFJZQmhsNkVDakYwUHRacTE1S0JVMEVMZ01paE1QaFVDZ1FxRU0zQXVzM2xZclM3TVk3MWZMTGJacWt4REE2M1k1TXNsNHNQakNFbjRLcGUyRXdFVkFDWnk4R0lVVElVdmxhTXh5bHFpUVpNb2swcEZJOFh5YU9Wa2xxdHEvYVYzcllHemFsc0FoaGpTSE85cWN1M0E0NXpTaURoNEFnV3FmZGFzc0JjWUR6ZnRXVXUwcEY2bTdudDBucldwSXFpdExWUTZoWFB4VHdLYkFuZjJjcEQwUE5hVE1MSXMwc0JrZ1FlSjlWcnlvbHNKSWtTZFZtVTlFYWlsS3RLTzE4VjJOUURwSU1CUHdoTDFQQzArdXh3VVRNZ2c1RFpUb2pMaEk0QTRzeTgxRTBxMEthalpabXRBeUFxRWdWMGxZVXBha28rWHpjVVZVYTdRNTlEejBxQkVyd3VjZm1IVjRpWmF4NGFSZ2RtQXh1OTBDWVVRTkFJSVIwSkVNclNZWmtsTHZYbXVhQTlUZEJENmhZckt3MHRZd3ZHa1VLeHNGR0JhTUk3c1NHOGRMdzgwRUUydTN1TVlRNGc5clN0SXBVQWdhcDJsQWd3YklEMnFsU2dSd28rYlpHWkZsOXp4Z0NuTUhwTmQza0dpYldWcjZieVFiRlpZSUlNN05SMUFkeExrSDNrYVRyYTB3dlZsTXpxbHlJZkQ0UHc0N1VWMWQ5UGw5QStDa2NGbTRDaHV6ZDdsdlpmR2FEeldRQURhSXdGclFTeGtEQ05IQUdKZmF4S1g0RGlHNU1NUGdFUXk2TXdlWlhGRHNXdnB2NTUzbDR4OUpndGlRMjE2TFJhSll5RFhoZDQrbUhLbmNWcGMvUUxwc01Qak1TWG95RW0rM0c0OE16L0hNZDFuc3RDVHRTV0toQUNTUzRUeUdoaDhvbGJFazlDZ1gyZjZ1cmZTVkNyTWxpSkpCaGZmamVhaHVPQVFlVUdNNllabDgybzVOQkJPaXNWVHozV0N3WmJjVFoxczlXQlVWZkNaSHI5ZUV6UFFURFFGL05jUlY4dnRYNm9KZXdPa29UQmtTYmVhamJNUnFkanBKdjBMTTF6aERsTFJaU2JZVmg2QmtuR0R3NWtXZEU4SzFtQ0lsVmVnQkdxZE91U3ZDUXoyOXJWTlVwdVc1R29EdVIrdG9hVXBnTUlSaDBZdU8wZnBkemVwMDFWdDZVbUF4QTRIdXZxcVRSUXloMUs2Mm1KTWx3NWpYMTR2UHU3czBGYWNRLzlobFdvM3hNK0VOaG1OYWVJWFVZL2hwbzNieDRGZ3lBQUdzNjA3dGNobXExWk1EbUtBWXlYTUZNcUwzWVphVkdpTnp1TS9SR0hlNDRjRWk0NzFhSG5nd2lERUN3dGxZbndrbEdvK1NRWVZoWFNwcmMxbXE3b25SYVZHWE9JQ2pFbUJEYkp2ZmQ2c0FRY2l6UjZLUlYxR0V0R2F2d1RGOWR0VWpMMEVpSnFGcEVOeEYyQ3pldkM3UkxQbXowR0FLTXdXK0ZZZmo1OEZVYWNFRnJhNWtZNzB1RzFpSnE2M3AvbnhRL3ErUklFTHplZlExVklIUmp3NFFRYnVJWEU4NTdZZkI3ZWd3K3hrQzBFaC9SV2lxak9Zb1grb3ZkejhKSnIxa2RIaDdxZXA5QnBCck1kQzhNL2QwZUpKckxzSmFramc1cnFpV1NTRkJ5K3VLR24vOCtBZHd2VUFkR3dZUVFpVUFHcDRVOEREUGpCblo3ZllhelZJc3hTTmRxN2l4REtDa2U5U1ZBRFE0UER3b0Z5SU9Kd0NMQkdkaU11R3NkT0VNZ3hMc1NZOWpRSGVVS3RxWldLdmVpUmxWS2lINXhkSE5US0hDQ213SU11aUk5UllZTkFSR0lza0F3SFp6M29VT09iWlQ2RENvcFY1RkJTK1plb0FRM3B6ckJLZ0pBRVpHSXFxczBPZUNsMVNobmdBMkhKUWI3a0RxZ2xiNW1hRlNyeUpBOU00TmNLQW9HcENrV0RnNEthbWFESjJLVkQydXVROWg3NXpxWVgyYjBHTERUYktTQUFiZXJwUDdCN0tXdmJ3cEZXZ09HdEY3Yk84RFNVOS9Sd2N0MjRCYjJyVE1qWWxqN1FCcGdwVXFIZnZxQUFFY1h0V0t0Y01nTFZsOG9JRVZSRnpxc0Rlb3dMSU9GVE9OWGUvNnZHQ0NvdEF3eXlPUmZuNDdBUmVBZWVrSnA4UUFKZ0tHbUVsb3M3TlhVamY2RUdQQ1M5Njd6WURMNHYvQVNZekEwL1JNd1hORDkyTWt4MUlsYVFJU0NTazlPams5b3VxZ09EQWpyT2xqSnc3YzZDSVpyZW5GNmVsU2crL3NueCtmbmwrY0F3V0pBOFNsQWlFdy9BQjJjMzlHQmVja2d0ZFBUVTVnRUZCa3VMeS9QVDJqdDRHQ1A3Si9Bcy9Oam1OOFBWUWNXaUEvMHltaXBwNmZwMDRLNlQra0p5UEFDbDEwOE9FZ1RDa1FibDhmcUYxWWFsUTRqNlV2QXNKR0ZjMDlJTFgyYVRvTU8rN1NuUTQvaDh2S1lmdEdXUnRXWDdCYm1nMytBb1U2SzZiU3VwckVLTmNJWXpsa0VvS1h1OFhnY254REdJUFlhUHA5VkhTek9CK0dsZ00vSEE2SHJONFYwdXNnWkRpaUR3TDZFTXV5bGF3U2ZubEF5NktVK2cvZit2TVFnUktqVkdseHNNb0wwdndzSGFVZzFkRk5ZZEhFUHFyaFhKQlRyUDdodk5iZDhqQ0hFZGZnZmUrZmUwN2FTQlhBSmFTdGQ3U2V3QzdsVXUzM0lzU3ZoQjdWTUFxbFZ3SFFyREdvQWtZVGd2YWlrb2YwRDFKUldTcW9sTkZCRUkvRXEzM2ZQbWZHTFI5alVuZ1RFZWxwQ2lCMTdmajZQT1RPZU0wN2RWanNkOUtlSG40MThOc2NBWWdiTEc3Q0FmMytlR2h1Ym1ucnprakM4K1RSVHI5ZG5qamZNUEVFZ2ZTQWNuM0dIOEVFT3FWdVNRNGhoN0s5aXNWbWZxUk1JVUI4SU5ENWpzSUVJbitwMStBL2xlTDA1VFJHZURiL0FqcHpMQUhLNGhURUJiN3pZdCtxM1k1K1I0WkF3b0JtL2ZFbnFEd1JRekU4Ykc4Zkg2K3ZyaDIrRGNRMndCUnA1Z3hRZTliMGZkNVZoKzBzYWhIQTQwNlFNVVA3bEFyeEJoc09aalhWU3FCeUlYeDBtNHhxRUFjY3JVLzBlNS9QdVl0RmgreGZEMjl2YjAybVFRZDA4Um9aUEx3TVJnSFlkMTVzdVFpdjlCZlhvR1RGbzkyNFF0WVpJdWpUQWtBRVFWcmQvUUJObnB2WGpHZmozNlM4emJkWVJBWXo1c0htNHNiNjMxMXB2dFp5eDdZdmpyZFNpSC9iZkhrTGpyVStmVUlUdDFlWGxqMXRiSDR0T0MvVCsrTEJac20yelZEZzlPajNkQXhuQUwvR3d0ZE15UVpXR2gzMkdweUdFd2Y3Mkh3WkRnZ0NMUUlMVlg3OStmVmorc0x4bHRsckg2eWVtbGkxbG05bDBTU3NXRDdHQWpOSW5Pd2NqSUladFlnd2VBMjNmb280WngyTHdJTkNxVjREaDQ2OFB0QndldEZzbkIwMDdteTNsc21rejI4eGxqWndBZjVTQUFTeDZtRFlOYUE2ZU1UeEtQYnlGY2UrTGQwVlhWbFkvQXNNYUxjMkRkdHN4YkFNUVFBNVFiTTBRNEZmYWJKVkhWb2VIZ3lDRGpJNGhCWjFBMDljMkx1ZlBsNkZ6ZlZaUURFZHJHNlNzclIwNHJhWnRHeUFFRTYyOFdUSkxKc1JQOXA3Y2ZMdnRtZ0lHR2Y5OCt1U1JidzNBVU9vbkF6YzZHSkxFa3ljTElJY1BSNmQ3ZTBkN1IwZEhlMnQxeHdFR1UxV1VzOWJKeVFHV2s1MmRYYjR3c2tLSDdGKzRjVktnU2FuVTR5ajNkbVBNSTNQdnNaTnBQK0NaQ01QYUVhNHYxTmhiQjBrMDlZT21ZMXFOeG00TGZORU9MYnpTbk40T05NbHJvVWtUSGZNZWU3UzVEb0UyL2Jtd1FCazI5aHA4NDZ5MURoQ09venRDODd6UmFKeEI1ZUZYNDN5bnpXZkdWc2hndDk4dzBDQ0RUb21MTklVcHpyeVo3R2hvenNtZi8xaFljU0VVa0VNTEdBNGNVQituZmI2N3U5dmVnZHJ6RFNDUnpXbDZPelRNNEU0bUE0WkpqdXVuYjZWemYzeVRRQWhrQUloVC9yVFYydGc0Y1NxVnlvRitnaERuWjQwR3orL3U4amFaYmhMTW1YRmJCbVNJTy9jbjB2eWw3S1EvSFpIZXBhYUNnUFpZT1Q4OTNUaXAxR3ExdWNwQnBRMFF1OGpBODNJNlQyWHd3dGNqRHlFVlVRd3g1OEtWUmk5TVNLVGFCQlFvQ3Y1b3ZWTDdCcVcyVkVGUkFJTWlaOHdwNmxDaGNmYXRPWmhhK1RnWGpTSFcvTmJRdEVwd1Q0ODhDS1JRZU1VaURPMTJwZUk0QjRaUk10TmoweXNRcG50MlFCRlM3Z3pYd2NoekVnZVl6QTFOVVgyaXpvbFFnRmtjV2J1N1oyZTc0S2NVNjdnK01UR2QvK0xlakE0anVBeVBCNlBQRFkyYkg1Y0w1dWlTVzcxRUZFQ0JHTTVTR3hHdzdIMzR1THF5OHVJSkxaNGhYRkNreUpPK0dlUU92QTBZdkhhQ1VMeHhIR0lRN2JQemMrdm9sODhReUNBUXcrQmdqQXhHRnZrUHVXbDNlaVdaZDc5QUtiYUtUYWN5VjZzU2svaFdjWDRBdzRJSFFPdFAyeldxUjNrR2M5Yi9pSldIVXBwMForcTZEQXY1MTZZSkNIUFZhclZXK3dZY0ZYUHFCekNFRVZKZWpKUWF6TWRLVEdhVlMvUGFiU3B3VXRhZjAyTnBCd293WUttT296QXF0am4xWmNGWElWOEdxRVozSW9jREZjckxwWGs0T1dVNldQZWxwYVU1dDlUR3g4ZmI0dUhJWktCR0M1NE1CbU9uOGpMTUY4MU4wMVNya2FaTGdBV1VDUUtPSmFjQ0ZHZjJTSDRCVklnMEpwNFU4aXh5bWhqbStMMmVCSVl4QitxUC82dFYvS21DS2kwNVZVQ1luVzNySTVPK0VkQU1PUllaK296ekZGOVBUcWJwOWErTjE2cExXSFd2ekk3UHppdmEyTU5VWUFtampITDhHT2NlNTE2bmlTR00xMnB3L2NmSGE2VDZwTXpQejF0V2VwSW1ZNUVjSUVZNTdUMVlUNkFDREZqNUd1b1JLVlVrbUoyMTV4ZGxNMDhBVXFPUDh5VldKK3hGTHY3UENqQUF4TnhzZFpZSVlaNlVSZjNkb21ybTNkeGpoa3RWOUdaTmhFb1ZsY2lacmM3NmFyUTR2MWo0aVF3MEI1emwwZ2c5V2w5REFuT2VlK1VERUlSRlZYcS9tTWI4OEdtMml6djBiSDBOYWE3Q3pmdHFSQXBsR00yelh2R2tsK3RyU0VzaGd2ZUw3OFhzMTYvcGZBL1dwdWpoV2kzU3o0QUFpMmgrZmZlSzQzckEwTXUxaTZSM0FRRXd2UHJlazdYbmVyMTJFVks0Q1AvUnVONlVQcXkvOVBQN095amZmL1pzL2I5N3NTN2N3SVA3czViWFFMSXUzRjJRUTdKTzRoMlJ3MENpUzRsdlpXUVBpVys5RzIxYzBqN2NzazBuNnhuZkpkLzZJSWsxN29ZY2t2WWhhYWRaK2RiL3dTQm8ydTg4bDBEVHJ6N1VRN3B1WEVNU1dNaWhtNzdvdm9MM3lSVzEyeE1LUEgvMTRTcnExYzgwOGJvZGU5Ri9rQ3p2K1hQZG5rOVNydGxWNU1WTG45aDRYZlMreE40cXo0dWF4QWxsdWV2SGpRalhLTjVWQnBGbjhtQ29MdG80amI5eThramxLb1BDWi9yVUJ3SXhoS3hSMnBkNVJTU1hXU2pnVzlBRkFTeEZWT0JGS2lpOGlFWlRzRFJ2QjkrSUNJTWdXM2dFQzc1VkJtT1FSVkdpaHlTN1plU3lJU3BscmlCcnRnaWZTZFJvRk40cUNMRmlQamhZU0VsazBHR3dqREo1cTFqd3A4RmxvREpvTGlKNVZlRE1QRjVoZ1hnQy85dUVRZWQ1YWwwYTUxcVpBT2FtV1BCamtGMFV0RHIzZUVSd0daNmM4UWF0NjBJT01pOWZVR0dORTBTc2FKa2NWeWlUMDFnU3lrczJ1QUxQMnk0RE1HcWNwSmN2TWVBQmlNMkQ5eW9JRW53REgzS1Z3YTBvR2QzQXIvUDdFcHlGbHpnRGRwSTRXN25CbjNRUkw0VVo0TFRvUnlRVVJJYjNIckNWUVdHZ2w5SGRUWVJCby90ZXRBZWRJSUlNOEJJVFVyQUtuUnFIaENJbmg2UWVVTWZEcWxTT0JTTGV5QXhXU0pjMDF6WmtPQW04VjhTQ1RobUNiYVJpK0VLcWNBMkQ0TnMzMVRnUW9nZ0Z2KzJaUFlqQk82RDcwZVdEL1NaRElXVFRkb2lCRTFSVWFWQ29qZ3hDZHd3cUxVS0lJWE9Kd2I3Z1dIN2JIalNpN2NRbm9YYnFWS1ZvVXlIcEN0R3E2eGc2NmRJbEJxcDhGOTF2aU1IVnBUTGFSb3orUTRGWW1xRHRLeUlhQnpGQXFJbWgwZE5tT2pDQWh1TytmblBYZ1FIZGhFQkNyR3NaTktKWDRBUEZXTzIwcEFheGhvWSszU0ppVUhsTExZaFlwdzRNTm5wSDY0cHZ2Y3lBN2xwRXR5eGR4MENpQlBDNGloQ3pINmNUWHk3akEyQU5mS3VnOU5IM2dacHBXRm1aT0N0eUhwa25GeFRWeVBiM3BlSlVzVjdFdjJTSTQ1R3BzZ25rR29sMkVCWmFaSU5CRDVqQlprWTA0c2F0K0tSZVEvSmphUCtTR0FaOUswbWhWL0tSY0dWZjl6TzZrL3MrZEtEUXgvNXZkMTh1T1BmOTd3TWxZelBKMkV6Q2tEeTNMQmt6Wmk2SGU4R1F0SEYzZ3VGZVBEOHUwYVdFSVdtbjc5ZTkzWUZrcnNOZHNJZGt6Z25IUFMvMWFtdmZiRG8zdEpudHpkYmY4YTN4YkxvNE5EVFJtNjM5aTcyWGg0WTJlN1AxZDNRcGxoemdVZzROUGUvRjFyNzRwVnl4T0RHeGpQVllucGdvRm5Qc3R2WXYxdGdjdWxBMjJXM3RuMDFQWEt6SEJMdXRmZlN0RXpkV0k4N1dQcmJUUVVVMmkyeTM5ckV2T25IamxZeXp0WC94MHUwenhJODF0cng2YkxIZTJyZitRNVpVZ2JqNUxOdXQvZXVMUGdlREJFMG9MbC9iM01iWjJqODVsSlova0d1WSs3R1pZN3YxLzZvUDlDQVo5MDdHWnBneVBFaHlMUk43U1B4U2tpK2FNTEMzNllHRUlXbW5HYlZ4eVp5VHBIMUlmT3Y5dXBjMWtPVDRKZTFEd25BLzJyaDd0TTVKRXJmZUQzdVFkRlZXK0FqRmlwMzF5aWptay9ZVnNTeEVTbUExZEZIZXZ3T3hoaUNMY1hLNUJWVVdicHZCOEhOK29wYXlZc1RScGZqeGtoQ25BdjVsRUdMTElVNDdMWmNaT0pkeTlIUndCZ3dadGJPbHE1YW1kSmxBTHU3Zm9qMTB0a2NaSEtmR1c5MXFwQlNaSVc3TXAzZTh6amJKT3V5NlpxSWVseUd5VFhkc29qU1ZGM1ZkdDBGTXVpRHBtRnV0Wit3YkxFS042NWNpeThIcTVKUVVQOGxVSUZuNjRIdDRtZStzZVlKOGF3d2R2eWVWZVZVUWRGQW9BYk5CRFVHRytndWRUVnhTNHNaODdCazRuU2E1SWdQYXE0MUlncXBFT0ZLdlk3N083aVRFZ0dxUzRTOHM4OENlSWJwTmQxYndTd3c2Zi9NeUtZWVZseUd5SE5SeWx3eVNndkdFMXRHRDZwSDlVdXgydW5QN1FDNTh3SUJyQWtBWFErNVoreERkSHFTTzBacWdndHVWQ2hySEZhaXNoSXlxbGlYV3JwVkp2TVJrUlIxT2pSRXZ4UjVma2hqRnJiY2FlOTk2LzRIRk9OOWQ2Y2ZGN0UrcnNmclRZcnorTktQeDFvd3M2cEV1cFNTVVJTVVRhMGt2ZHZleWROV0tNcnlreUtvZWMxR3krM0UvampEODdZKy8vN2Y2Y3NsdEVJYWlxSTJ4aWY4RzhsT1hnTUtFTUVKWlFHYk1XUkRyN2pOSkNGV2FkbENwNGwwR0VjSVpITjBQd3RWSUVXcW5KWThNUnJzS0tVUG50ZUdVVUdhczc1RXk5TjRhRmhtVUxRYWtERU5wRlRBUXBrUitRc3B3ellWaWhNUmh3bHFJQ2lvTnMwU21RcFFqU29ZMlFCMXVERXJzemhqWHRXb090eWlSdUs0MllEVGlBamJ3YkdLSVJteGRoN0FOUnpHdDBzT0lvc0dXcHJyWlJ4dnVESlRCTklVV1hhRmhsQjQyZ0JHUUp1MXhRYlJlUTVJeU1qTmtIQ3JoV3p4eHFsdS9GWW8vRVNCTlVBbWhRNFBsVmRjMVFRc293MWVHNklUZXV4RURSVDI2QW9MRTJSTGhIaWRwaitGdlg4Zi9NcW5qT1d5dFZQTWtMU0ZZYW9ROUJIY2QrbTZkemFpcmZqaTVjbWVGU2RrckFxRTBZMXdCaGM1TDc5M0hHdVdjTDNJTkJLODVlcllpVWtqQTBKdW9CSzY3a3ZrbTJTeCtGaWRlOU9aSmtqd2ZKWnNmL2orZFhCeUl0eG9BNUVSQXlSdUdTTUZTWll5VVVxeFFVaHFqVXZhZVlGNG80T0E4WGFNNFp3Q1FrZCtWVFNBckZmM0dnay9KOURwVTZxb09XZ0FBQUFCSlJVNUVya0pnZ2c9PSI+",
            "nft_type": "fire",
            "power": "3"
        }
    ]
]



*/