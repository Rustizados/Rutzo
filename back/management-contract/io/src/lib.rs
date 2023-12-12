#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type MainContractProgramId = ActorId;
pub type MainContractMetadata = String;
pub type NFTContractProgramId = ActorId;
pub type NFTContractMetadata = String;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitContract>;
    type Handle = InOut<ManagementAction, ManagementEvent>;
    type Others = ();
    type Signal = ();
    type Reply = ();    
    type State = InOut<ManagementStateQuery, ManagementStateReply>;
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Contract {
    pub owner: ActorId,
    pub main_contract_id: Option<MainContractProgramId>,
    pub main_contract_metadata: Option<MainContractMetadata>,
    pub main_contract_state: ContractState,
    pub nft_contract_id: Option<NFTContractProgramId>,
    pub nft_contract_metadata: Option<NFTContractMetadata>,
    pub nft_contract_state: ContractState,
    pub servers_under_maintenance: bool,
    pub allowed_users: Vec<ActorId>
}

impl Contract {
    pub fn is_approved(&self, caller: ActorId) -> bool {
        let is_approved = self.allowed_users
            .iter()
            .find(|&user| *user == caller)
            .is_some();
        self.owner == caller || is_approved
    }
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractState {
    #[default]
    Suspended,    
    Available,
    Maintenance
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContract {
    pub main_contract_id: Option<MainContractProgramId>,
    pub main_contract_metadata: Option<MainContractMetadata>,
    pub nft_contract_id: Option<NFTContractProgramId>,
    pub nft_contract_metadata: Option<NFTContractMetadata>,
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ManagementStateQuery {
    #[default]
    ServerIsUnderMaintenance,
    MainContractState,
    MainContractData,
    MainContractProgramId,
    MainContractMetadata,
    NFTContractState,    
    NFTContractData,
    NFTContractProgramId,
    NFTContractMetadata,
    All
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ManagementStateReply {
    ServersUnavailable,
    ServerIsUnderMaintenance(bool),
    MainContractState(ContractState),
    MainContractData {
        program_id: Option<MainContractProgramId>,
        metadata: Option<MainContractMetadata>
    },
    MainContractProgramId(Option<MainContractProgramId>),
    MainContractMetadata(Option<MainContractMetadata>),
    NFTContractState(ContractState),    
    NFTContractData {
        program_id: Option<NFTContractProgramId>,
        metadata: Option<NFTContractMetadata>
    },
    NFTContractProgramId(Option<NFTContractProgramId>),
    NFTContractMetadata(Option<NFTContractMetadata>),
    All(Contract)
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ManagementAction {
    SetServersUnderMaintenance(bool),
    SetMainContractData {
        program_id: MainContractProgramId,
        metadata: MainContractMetadata  
    },
    SetMainContractState(ContractState),
    SetMainContractProgramId(MainContractProgramId),
    SetMainContractMetadata(MainContractMetadata),
    SetNFTContractData {
        program_id: NFTContractProgramId,
        metadata: NFTContractMetadata
    },
    SetNFTContractState(ContractState),
    SetNFTContractProgramId(NFTContractProgramId),
    SetNFTContractMetadata(NFTContractMetadata),
    AllowUserToModifyContract(ActorId),
    RemoveUserToModifyContracT(ActorId)
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ManagementEvent {
    ServersUnderMaintenanceSet,
    MainContractDataSet,
    MainContractStateSet,
    MainContractProgramIdSet,
    MainContractMetadataSet,
    NFTContractDataSet,
    NFTContractStateSet,
    NFTContractProgramIdSet,
    NFTContractMetadataSet,
    UserAllowed(ActorId),
    UserRemoved(ActorId),
    UserDoesNotExists(ActorId),
    ErrorSettingMainContractData,
    ErrorSettingMainContractProgramId,
    ErrorSettingMainContractMetadata,
    ErrorSettingNFTContractData,
    ErrorSettingNFTContractProgramId,
    ErrorSettingNFTContractMetadata,
    UserNotAllowed(ActorId)
}

