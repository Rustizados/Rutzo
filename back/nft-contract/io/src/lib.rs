#![no_std]

use gear_lib_old::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
    delegated::DelegatedApproveMessage
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

use primitive_types::H256;

pub type UserNFTS = (ActorId, Vec<TokenMetadata>);

pub struct NFTMetadata;

impl Metadata for NFTMetadata {
    type Init = In<InitNFT>;
    type Handle = InOut<NFTAction, NFTEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<NFTStateQuery, NFTStateResponse>; //Out<IoNFT>;
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTStateQuery {
    TokensForOwner(ActorId),
    TokenById(TokenId),
    #[default]
    All
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTStateResponse {
    TokensForOwner(Vec<(TokenId, TokenMetadata)>),
    TokenData(Option<TokenMetadata>),
    All(IoNFT)
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Constraints {
    pub max_mint_count: Option<u32>,
    pub authorized_minters: Vec<ActorId>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitNFT {
    pub collection: Collection,
    pub royalties: Option<Royalties>,
    pub constraints: Constraints,
    pub main_contract: ActorId,
    pub tokens_metadata_default: Vec<TokenMetadata>
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Collection {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTAction {
    Mint {
        transaction_id: u64,
        token_metadata: TokenMetadata,
    },
    Burn {
        transaction_id: u64,
        token_id: TokenId,
    },
    Transfer {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    TransferPayout {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
        amount: u128,
    },
    NFTPayout {
        owner: ActorId,
        amount: u128,
    },
    Approve {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    DelegatedApprove {
        transaction_id: u64,
        message: DelegatedApproveMessage,
        signature: [u8; 64],
    },
    Owner {
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
    },
    Clear {
        transaction_hash: H256,
    },
    AddMinter {
        transaction_id: u64,
        minter_id: ActorId,
    },
    TranserNFTToUser {
        transaction_id: u64,        
        to: ActorId,
        token_id: TokenId
    },
    NFTData(TokenId),
    NFTDataFromTokensIdbyUser {
        tokens_id: Vec<TokenId>,
        user: ActorId
    },    
    NFTDataFromUsers(Vec<ActorId>),
    SetMainContract(ActorId),
    MintNFTsTo {
        to: ActorId,
        nfts: Vec<TokenMetadata>
    },
    MintDefaultNFT {
        transaction_id: u64,
        to: ActorId,
        nft_id: u8
    },
    BurnAllNFTS,
    DeleteContract,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    NFTPayout(Payout),
    Approval(NFTApproval),
    Owner {
        owner: ActorId,
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
        approved: bool,
    },
    MinterAdded {
        minter_id: ActorId,
    },
    NFTData(Option<TokenMetadata>),
    NFTsData(Vec<(TokenId, Option<TokenMetadata>)>),
    TokenIdNotExists(TokenId),
    TokenDefaultIdNotExists(u8),
    AllNFTInformation(Vec<(ActorId, Vec<TokenMetadata>)>),
    ActionOnlyForMainContract,
    MainContractIsNotApproved,
    MainContractSet,
    NFTsMinted,    
    AllBurned
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNFTState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub royalties: Option<Royalties>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNFT {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>
}

impl From<&NFTState> for IoNFTState {
    fn from(value: &NFTState) -> Self {
        let NFTState {
            name,
            symbol,
            base_uri,
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties,
        } = value;

        let owner_by_id = owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*hash, *actor_id))
            .collect();

        let token_approvals = token_approvals
            .iter()
            .map(|(key, approvals)| (*key, approvals.iter().copied().collect()))
            .collect();

        let token_metadata_by_id = token_metadata_by_id
            .iter()
            .map(|(id, metadata)| (*id, metadata.clone()))
            .collect();

        let tokens_for_owner = tokens_for_owner
            .iter()
            .map(|(id, tokens)| (*id, tokens.clone()))
            .collect();

        Self {
            name: name.clone(),
            symbol: symbol.clone(),
            base_uri: base_uri.clone(),
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties: royalties.clone(),
        }
    }
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Nft {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub media_url: String,
    pub attrib_url: String,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub tokens: Vec<(TokenId, Nft)>,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>,
    pub owners: Vec<(ActorId, TokenId)>,
    pub collection: Collection,
    pub nonce: TokenId,
    pub constraints: Constraints,
}
