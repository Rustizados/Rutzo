#![no_std]

use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::*,
    state::*,
};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};
use primitive_types::{ H256 };

pub type NftId = u128;
pub type OwnerActualTransactionId = u64;

pub struct NFTMetadata;

impl Metadata for NFTMetadata {
    type Init = In<InitNFT>;
    type Handle = InOut<NFTAction, NFTEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RutzoNft {
    pub media: String,
    pub nft_type: String,
    pub power: u8
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Config {
    pub max_mint_count: Option<u32>,
    pub authorized_minters: Vec<ActorId>,
}

//#[derive(Debug, Encode, Decode, TypeInfo)]
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TokenURI {
    /// Token metadata derived from gear-lib
    pub metadata: TokenMetadata,
    /// base64encoded NFT.
    pub content: RutzoNft
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitNFT {
    /// NFT name - Rutzo NFT
    pub name: String,
    /// NFT symbol - Rutzo
    pub symbol: String,
    // NFT base_uri (not applicable in on-chain)
    pub base_uri: String,
    /// Base Image is base64encoded image.
    pub base_image: String,
    /// Royalties for NFT
    pub royalties: Option<Royalties>,
    /// NFT vector - each item containes a base64encoded image
    /// and TokenMetadata for each NFT
    pub nfts: Vec<(TokenMetadata, RutzoNft)>,
    /// Defaults NFT to mint to new users
    pub default_nfts: Vec<(TokenMetadata, RutzoNft)>,
    /// Marketplace ContractId for sale NFT's
    pub marketplace_actorid: Option<ActorId>,
    pub main_contract: Option<ActorId>
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
    SetMainContract {
        main_contract_id: ActorId  
    },
    ApproveMarketplace {
        transaction_id: u64,
        marketplace_id: ActorId
    },
    MintDefault {
        // Mint default three NFT's to new users.
        transaction_id: u64,
        to: ActorId,
    },
    Mint {
        // Modificado parte del mint ----------------
        // Como es agregar un nuevo NFT, el dueño sera el propio contrato.
        transaction_id: u64,
        nft_data: RutzoNft,
        token_metadata: TokenMetadata,
    },
    Burn {
        // Modificado esta parte del burn -------------------
        // Como es quemar un NFT, se especifica el token_id del nft.
        transaction_id: u64,
        token_id: TokenId
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
    NFTDataFromTokenId {
        tokens_id: Vec<TokenId>
    }
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    NFTPayout(Payout),
    Approval(NFTApproval),
    MarketplaceApproval((u64, NFTApproval)),
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
    MainContractEstablished,
    MarketplaceApproved,
    UserNotAllowedToMint,
    UserIsNotTheOwner,
    NoMainContractInActualContract,  
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
pub struct State {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub base_image: String,
    pub nfts: Vec<(TokenId, RutzoNft)>,
    pub nft_available: Vec<TokenId>,
    pub given_default_nfts: Vec<ActorId>,
    pub default_nfts: Vec<(TokenMetadata, RutzoNft)>,
    pub transactions: Vec<(H256, NFTEvent)>,
    pub config: Option<Config>,
    pub marketplace_id: Option<ActorId>,
    pub main_contract: Option<ActorId>
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

impl IoNFTState {
    pub fn token(&self, token_id: TokenId) -> Token {
        let mut token = Token::default();
        if let Some((_, owner_id)) = self.owner_by_id.iter().find(|(id, _)| token_id.eq(id)) {
            token.id = token_id;
            token.owner_id = *owner_id;
        }
        if let Some((_, approved_account_ids)) =
            self.token_approvals.iter().find(|(id, _)| token_id.eq(id))
        {
            token.approved_account_ids = approved_account_ids.iter().copied().collect();
        }
        if let Some((_, Some(metadata))) = self
            .token_metadata_by_id
            .iter()
            .find(|(id, _)| token_id.eq(id))
        {
            token.name = metadata.name.clone();
            token.description = metadata.description.clone();
            token.media = metadata.media.clone();
            token.reference = metadata.reference.clone();
        }
        token
    }

    pub fn tokens_for_owner(&self, owner: &ActorId) -> Vec<Token> {
        let mut tokens = vec![];

        if let Some((_owner, token_ids)) = self.tokens_for_owner.iter().find(|(id, _)| owner.eq(id))
        {
            for token_id in token_ids {
                tokens.push(self.token(*token_id))
            }
        }
        tokens
    }

    pub fn total_supply(&self) -> u128 {
        self.owner_by_id.len() as u128
    }

    pub fn supply_for_owner(&self, owner: &ActorId) -> u128 {
        if let Some((_owner, tokens)) = self.tokens_for_owner.iter().find(|(id, _)| owner.eq(id)) {
            tokens.len() as u128
        } else {
            0
        }
    }

    pub fn all_tokens(&self) -> Vec<Token> {
        self.owner_by_id
            .iter()
            .map(|(token_id, _toks)| self.token(*token_id))
            .collect()
    }

    pub fn approved_tokens(&self, account: &ActorId) -> Vec<Token> {
        self.owner_by_id
            .iter()
            .filter_map(|(id, _)| {
                self.token_approvals
                    .iter()
                    .find(|(token_id, _)| id.eq(token_id))
                    .and_then(|(id, approvals)| {
                        approvals.contains(account).then_some(self.token(*id))
                    })
            })
            .collect()
    }
}





/*
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNft {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>,
}
*/


/*
#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNft {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>,
}
*/



/*
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
*/


/*
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
    pub config: Config,
}
*/