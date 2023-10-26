mod utils;

use gear_lib::non_fungible_token::io::*;
use gear_lib::non_fungible_token::token::*;
use gstd::{prelude::*, debug};
use gtest::{System, Log};
use program_io::*;

const USERS: &[u64] = &[3, 4, 5, 9, 10, 11];
const ZERO_ID: u64 = 0;
const MARKETPLACE_ID: u64 = 6;
const MAINCONTRACT_ID: u64 = 7;

// First action -----------------------------------

#[test]
fn setting_main_contract_id() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
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
}

#[test]
fn action_failed_without_main_contract() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    let res = utils::mint(
        &nft, 
        USERS[0], 
        transaction_id,
        nft_test.0,
        nft_test.1
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::NoMainContractInActualContract);
    
    assert!(res.contains(&expected_log));
}

#[test]
fn action_success_after_setting_main_contract() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 3.into(),
    }).encode();
    
    let res = utils::mint(
        &nft, 
        USERS[0], 
        transaction_id,
        nft_test.0,
        nft_test.1
    );
    
    assert!(res.contains(&(USERS[0], message)));
}

#[test]
fn action_failed_user_is_not_the_owner_after_setting_main_contract() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let mut res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MARKETPLACE_ID.into()
        }
    );
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));

    res = utils::mint(
        &nft, 
        USERS[2], 
        transaction_id,
        nft_test.0,
        nft_test.1
    );
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(NFTEvent::UserIsNotTheOwner);
    
    assert!(res.contains(&expected_log));
}


// Second action ------------------------------------------


#[test]
fn setting_markeplace_id() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
}

#[test]
fn setting_markeplace_id_fail() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    let res = nft.send(
        USERS[3],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[3])
        .payload(NFTEvent::UserIsNotTheOwner);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
}

#[test]
fn approve_of_marketplace_success() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    // Check pre - minted nft to contract
    
    utils::check_token_from_state(&nft, USERS[0], 0);
    
    // Default NFTs must have approved "marketplace" contract
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::IsApproved {
            to: MARKETPLACE_ID.into(),
            token_id: 0.into(   ),
            approved: true
        });
    
    let res = nft.send(
        USERS[0],
        NFTAction::IsApproved {
            to: MARKETPLACE_ID.into(),
            token_id: 0.into()
        }
    );           
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
}

#[test]
fn approve_of_marketplace_success_fail() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    
    let res = nft.send(
        USERS[2],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[2])
        .payload(NFTEvent::UserIsNotTheOwner);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
}


// Third action ----------------------------------------

#[test]
fn mint_default() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    transaction_id += 1;
    let res = nft.send(
        MAINCONTRACT_ID,
        NFTAction::MintDefault { 
            transaction_id: transaction_id.into(),
            to: USERS[2].into(),
        } 
    );
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: MAINCONTRACT_ID.into(),
        to: USERS[2].into(),
        token_id: 0.into(),
    }).encode();
    
    assert!(!res.main_failed());
    assert!(res.contains(&(MAINCONTRACT_ID, message)));
    
    utils::check_token_from_state(&nft, USERS[2], 3);
    utils::check_token_from_state(&nft, USERS[2], 4);
}

#[test]
fn mint_default_fail() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MAINCONTRACT_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    // must fail since the user is not the owner or autorized
    transaction_id += 1;
    let res = nft.send(
        USERS[2],
        NFTAction::MintDefault { 
            transaction_id: transaction_id.into(),
            to: USERS[2].into(),
        } 
    );
    
    let expected_log = Log::builder()
        .dest(USERS[2])
        .payload(NFTEvent::UserNotAllowedToMint);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
}





#[test]
fn transfer_success_marketplace() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    transaction_id += 1;
    
    // Check NFT owner is from the correct user 
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 0, transaction_id);
    assert!(!res.main_failed());
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    }).encode();
    
    assert!(!res.main_failed());
    assert!(res.contains(&(MARKETPLACE_ID, message)));

    utils::check_token_from_state(&nft, USERS[1], 0);
    
}

#[test]
fn transfer_success_marketplace_fail() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    let res = nft.send(
        USERS[0],
        NFTAction::ApproveMarketplace {
            transaction_id: transaction_id.into(),
            marketplace_id: MARKETPLACE_ID.into()
        }
    );
    
    let expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MarketplaceApproved);
    
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());
    
    // Check NFT owner is from the correct user 
    
    utils::check_token_from_state(&nft, USERS[0], 0);

    transaction_id += 1;
    let res = utils::transfer(&nft, USERS[2], USERS[1], 0, transaction_id);
    
    // must fail since is not the owner or approvated user.
    assert!(res.main_failed());
    
    transaction_id += 1;
    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 10, transaction_id);
    
    // must fail since the NFT does not exists
    assert!(res.main_failed());
    
    // Correct transaction
    transaction_id += 1;
    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 0, transaction_id);
    assert!(!res.main_failed());
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    }).encode();
    
    assert!(!res.main_failed());
    assert!(res.contains(&(MARKETPLACE_ID, message)));

    utils::check_token_from_state(&nft, USERS[1], 0);
    
    // transfer agarin with marketplade id will fail
    transaction_id += 1;
    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 0, transaction_id);
    assert!(res.main_failed());
    
    // Must fail since is not the owner
    transaction_id += 1;
    let res = utils::transfer(&nft, USERS[2], USERS[1], 0, transaction_id);
    assert!(res.main_failed());
}






//    Normal tests, before setting main_contract   ------------------

/*
#[test]
fn mint_success() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 3.into(),
    }).encode();
    
    let res = utils::mint(
        &nft, 
        USERS[0], 
        transaction_id,
        nft_test.0,
        nft_test.1
    );
    
    assert!(res.contains(&(USERS[0], message)));
    
    // Check that we minted a token properly    
    utils::check_token_from_state(&nft, USERS[0], 3);
}

#[test]
fn mint_failures() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
    let mut res = nft.send(
        USERS[0],
        NFTAction::SetMainContract {
            main_contract_id: MARKETPLACE_ID.into()
        }
    );
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(NFTEvent::MainContractEstablished);
    
    assert!(res.contains(&expected_log));

    res = utils::mint(
        &nft, 
        USERS[2], 
        transaction_id,
        nft_test.0.clone(),
        nft_test.1.clone()
    );
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(NFTEvent::UserIsNotTheOwner);
    
    assert!(res.contains(&expected_log));
}

#[test]
fn burn_success() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    // Check pre - minted nft to contract
    
    utils::check_token_from_state(&nft, USERS[0], 0);
    
    let res = utils::burn(
        &nft, 
        USERS[0], 
        0,
        transaction_id,
    );
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: ZERO_ID.into(),
        token_id: 0.into(),
    })
    .encode();

    assert!(res.contains(&(USERS[0], message)));
    
    // We should check against owner_id = 0 since the token is burned
    // must fail since nft with token id does not exists
    assert!(!utils::check_token_from_state_bool(&nft, 0, 0));
}

#[test]
fn burn_failures() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    // must fail since the token doesn't exists
    assert!(utils::burn(
        &nft, 
        USERS[0], 
        5,
        transaction_id,
    ).main_failed());
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[0], 0);
    
    // must fail since the caller is not the owner of the token
    let res = utils::burn(
        &nft, 
        USERS[1], 
        0,
        transaction_id,
    );
    
    let expected_log = Log::builder()
        .dest(USERS[1])
        .payload(NFTEvent::UserIsNotTheOwner);
    
    assert!(res.contains(&expected_log));
}

*/





















/*
#[test]
fn approve_success() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::approve(&nft, USERS[0], USERS[1], 0);
    let message = OnChainNFTEvent::Approval(NFTApproval {
        owner: USERS[0].into(),
        approved_account: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    assert!(!utils::transfer(&nft, USERS[1], USERS[2], 0).main_failed());
}

#[test]
fn approve_failures() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    assert!(!utils::mint(&nft, USERS[0], vec![0, 1]).main_failed());
    // must fail since the token doesn't exist
    assert!(utils::approve(&nft, USERS[0], USERS[1], 1).main_failed());
    // must fail since the caller is not the token owner
    assert!(utils::approve(&nft, USERS[1], USERS[0], 0).main_failed());
    // must fail since approval to the zero address
    assert!(utils::approve(&nft, USERS[1], ZERO_ID, 0).main_failed());

    //approve
    assert!(!utils::approve(&nft, USERS[0], USERS[1], 0).main_failed());
    //transfer
    assert!(!utils::transfer(&nft, USERS[1], USERS[2], 0).main_failed());
    //must fail since approval was removed after transferring
    assert!(utils::transfer(&nft, USERS[1], USERS[0], 0).main_failed());
}

*/






/*

#[test]
fn transfer_success_main_contract() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 0, transaction_id);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(!res.main_failed());
    assert!(res.contains(&(MARKETPLACE_ID, message)));
    
    println!("\n Checando si se transfirio a 1 !! \n");B

    // Check the token now belongs to another user
    utils::check_token_from_state(&nft, USERS[1], 0);
    
    transaction_id += 1;    
    let res = utils::transfer(&nft, MAINCONTRACT_ID, USERS[2], 0, transaction_id);
    
    assert!(!res.main_failed());
    
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[1].into(),
        to: USERS[2].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(MAINCONTRACT_ID, message)));
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[2], 0);
}


#[test]
fn transfer_success_user() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[0], 0);

    let res = utils::transfer(&nft, MARKETPLACE_ID, USERS[1], 0, transaction_id);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[0].into(),
        to: USERS[1].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(MARKETPLACE_ID, message)));

    // Check the token now belongs to another user
    utils::check_token_from_state(&nft, USERS[1], 0);
    
    transaction_id += 1;
    let res = utils::transfer(&nft, USERS[1], USERS[2], 0, transaction_id);
    let message = NFTEvent::Transfer(NFTTransfer {
        from: USERS[1].into(),
        to: USERS[2].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[1], message)));
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[2], 0);
}

#[test]
fn transfer_failures() {
    let sys = System::new();
    let mut transaction_id: u64 = 1;
    utils::init_nft_no_marketplace(&sys);
    let nft = sys.get_program(1);
    let nft_test = utils::one_nft();
    
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
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[0], 0);
    
    // must fail since the token doesn't exists
    assert!(utils::transfer(
        &nft, 
        MARKETPLACE_ID, 
        USERS[1], 
        10, 
    transaction_id).main_failed());
    
    // must fail since the nft owner is the contract owner
    // and the caller is not the markeplace contract
    
    transaction_id += 1;
    assert!(utils::transfer(
        &nft, 
        USERS[0], 
        USERS[1], 
        10, 
    transaction_id).main_failed());
    
    // Correct transfer
    transaction_id += 1;
    assert!(utils::transfer(
        &nft, 
        MARKETPLACE_ID, 
        USERS[1], 
        0, 
    transaction_id).main_failed());
    
    // Check NFT owner is from the correct user   
    utils::check_token_from_state(&nft, USERS[1], 0);
    
    // must fail since the caller is not the token owner
    transaction_id += 1;
    assert!(utils::transfer(
        &nft, 
        USERS[2], 
        USERS[0], 
        0, 
    transaction_id).main_failed());
    
    // must fail since transfer to the zero address
    // the owner is a user
    transaction_id += 1;
    assert!(utils::transfer(
        &nft, 
        USERS[1], 
        ZERO_ID, 
        0, 
    transaction_id).main_failed());
    
    // must fail since transfer to the zero address
    // the owner is the owner contract
    transaction_id += 1;
    assert!(utils::transfer(
        &nft, 
        MARKETPLACE_ID, 
        ZERO_ID, 
        0, 
    transaction_id).main_failed());
}







#[test]
fn test_token_uri_state() {
    let sys = System::new();
    utils::init_nft(&sys);
    let nft = sys.get_program(1);
    let res = utils::mint(&nft, USERS[0], vec![0, 1]);
    let message = OnChainNFTEvent::Transfer(NFTTransfer {
        from: ZERO_ID.into(),
        to: USERS[0].into(),
        token_id: 0.into(),
    })
    .encode();
    assert!(res.contains(&(USERS[0], message)));
    // Check that we minted a token properly
    utils::check_token_from_state(&nft, USERS[0], 0);

    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };
    let content = vec![String::from("PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+"), String::from("PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4=")];
    utils::check_token_uri(&nft, 0, token_metadata, content);
}
*/