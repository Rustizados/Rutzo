#![no_std]

use gstd::{
    prelude::*, 
    msg
};
use management_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern "C" fn init() {
    let InitContract {
        main_contract_id,
        main_contract_metadata,
        nft_contract_id,
        nft_contract_metadata,
    } = msg::load()
        .expect("Error decoding init message");
        
    let mut contract = Contract::default();
    
    if main_contract_id.is_some() && main_contract_metadata.clone().is_some() {
        contract.main_contract_state = ContractState::Available;
    }
    
    if nft_contract_id.is_some() && nft_contract_metadata.clone().is_some() {
        contract.nft_contract_state = ContractState::Available;
    }
    
    contract.servers_under_maintenance = true;
    
    if contract.nft_contract_state ==  ContractState::Available || contract.main_contract_state == ContractState::Available {
        contract.servers_under_maintenance = false;
    }
    
    contract.owner = msg::source();
    contract.main_contract_id = main_contract_id;
    contract.main_contract_metadata = main_contract_metadata;
    contract.nft_contract_id = nft_contract_id;
    contract.nft_contract_metadata = nft_contract_metadata;
    
    unsafe {
        CONTRACT = Some(contract);
    };
}

#[no_mangle]
extern "C" fn handle() {
    let message = msg::load()
        .expect("Error decoding");
    let caller = msg::source();
    let contract = state_mut();
    
    match message {
        ManagementAction::SetServersUnderMaintenance(in_maintenance) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.servers_under_maintenance = in_maintenance;
            
            msg::reply(ManagementEvent::ServersUnderMaintenanceSet, 0)
                .expect("Error in reply 'ManagementEvent::ServersUnderMaintenanceSet'");
        },
        ManagementAction::SetMainContractData {
            program_id,
            metadata  
        } => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.main_contract_id = Some(program_id);
            contract.main_contract_metadata = Some(metadata);
            
            msg::reply(ManagementEvent::MainContractDataSet, 0)
                .expect("Error in reply 'ManagementEvent::MainContractDataSet'");
        },
        ManagementAction::SetMainContractState(contract_state) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.main_contract_state = contract_state;
            
            msg::reply(ManagementEvent::MainContractStateSet, 0)
                .expect("Error in reply 'ManagementEvent::MainContractStateSet'");
        },
        ManagementAction::SetMainContractProgramId(program_id) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.main_contract_id = Some(program_id);
            
            msg::reply(ManagementEvent::MainContractProgramIdSet, 0)
                .expect("Error in reply 'ManagementEvent::MainContractProgramIdSet'");
        },
        ManagementAction::SetMainContractMetadata(metadata) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }

            contract.main_contract_metadata = Some(metadata);
            
            msg::reply(ManagementEvent::MainContractMetadataSet, 0)
                .expect("Error in reply 'ManagementEvent::MainContractMetadataSet'");
        },
        ManagementAction::SetNFTContractData {
            program_id,
            metadata
        } => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.nft_contract_id = Some(program_id);
            contract.nft_contract_metadata = Some(metadata);
            
            msg::reply(ManagementEvent::NFTContractDataSet, 0)
                .expect("Error in reply 'ManagementEvent::NFTContractDataSet'");
        },
        ManagementAction::SetNFTContractState(contract_state) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.nft_contract_state = contract_state;
            
            msg::reply(ManagementEvent::NFTContractStateSet, 0)
                .expect("Error in reply 'ManagementEvent::NFTContractStateSet'");
        },
        ManagementAction::SetNFTContractProgramId(program_id) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.nft_contract_id = Some(program_id);
            
            msg::reply(ManagementEvent::NFTContractProgramIdSet, 0)
                .expect("Error in reply 'ManagementEvent::NFTContractProgramIdSet'");
        },
        ManagementAction::SetNFTContractMetadata(metadata) => {
            if !contract.is_approved(caller) {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.nft_contract_metadata = Some(metadata);
            
            msg::reply(ManagementEvent::NFTContractMetadataSet, 0)
                .expect("Error in reply 'ManagementEvent::NFTContractMetadataSet'");
        },
        ManagementAction::AllowUserToModifyContract(user_id) => {
            if caller != contract.owner {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.allowed_users.push(user_id);
            
            msg::reply(ManagementEvent::UserAllowed(user_id), 0)
                .expect("Error in reply 'ManagementEvent::UserAllowed'");
        },
        ManagementAction::RemoveUserToModifyContracT(user_id) => {
            if caller != contract.owner {
                msg::reply(ManagementEvent::UserNotAllowed(caller), 0)
                    .expect("Error in reply 'ManagementEvent::UserNotAllowed'");
                return;
            }
            
            contract.allowed_users = contract.allowed_users.clone()
                .into_iter()
                .filter(|address| *address != user_id)
                .collect();
            
            msg::reply(ManagementEvent::UserRemoved(user_id), 0)
                .expect("Error in reply 'ManagementEvent::UserAllowed'");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn state() {  
    let state_query = msg::load()
        .expect("Error decoding 'ManagementStateQuery'");
    let contract = state_ref();
    
    let response;
    
    match state_query {
        ManagementStateQuery::ServerIsUnderMaintenance => {
            response = ManagementStateReply::ServerIsUnderMaintenance(contract.servers_under_maintenance);
        },
        ManagementStateQuery::MainContractState => {
            response = ManagementStateReply::MainContractState(contract.main_contract_state.clone());
        },
        ManagementStateQuery::MainContractData => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::MainContractData {
                    program_id: contract.main_contract_id,
                    metadata: contract.main_contract_metadata.clone()
                }
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::MainContractProgramId => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::MainContractProgramId(contract.main_contract_id)
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::MainContractMetadata => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::MainContractMetadata(contract.main_contract_metadata.clone())
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::NFTContractState => {
            response = ManagementStateReply::NFTContractState(contract.nft_contract_state.clone());
        },    
        ManagementStateQuery::NFTContractData => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::NFTContractData {
                    program_id: contract.nft_contract_id,
                    metadata: contract.nft_contract_metadata.clone()
                }
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::NFTContractProgramId => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::NFTContractProgramId(contract.nft_contract_id)
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::NFTContractMetadata => {
            response = if !contract.servers_under_maintenance {
                ManagementStateReply::NFTContractMetadata(contract.nft_contract_metadata.clone())
            } else {
                ManagementStateReply::ServersUnavailable
            };
        },
        ManagementStateQuery::All => {
            response = ManagementStateReply::All(contract.clone());
        }
    }
    
    msg::reply(response, 0)
        .expect("Error in sending state reply 'ManagementStateReply'");
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