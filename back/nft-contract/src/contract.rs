use gear_lib_old::non_fungible_token::{
    io::NFTTransfer,
    nft_core::*, 
    state::*, 
    token::*,
};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gstd::{
    errors::Result as GstdResult, 
    exec, 
    msg, 
    prelude::*, 
    ActorId, 
    MessageId,
    Vec,
    collections::HashMap
};
//use nft_io::*;

use nft_io::{
    Collection, Constraints, InitNFT, IoNFT, NFTAction, NFTEvent, Nft, State
};
use primitive_types::{H256, U256};

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct Contract {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: HashMap<H256, NFTEvent>,
    pub collection: Collection,
    pub constraints: Constraints,
    pub main_contract: ActorId
}

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode InitNFT");
    if config.royalties.is_some() {
        config.royalties.as_ref().expect("Unable to g").validate();
    }
    let nft = Contract {
        token: NFTState {
            name: config.collection.name.clone(),
            royalties: config.royalties,
            ..Default::default()
        },
        collection: config.collection,
        constraints: config.constraints,
        owner: msg::source(),
        main_contract: config.main_contract,
        ..Default::default()
    };
    CONTRACT = Some(nft);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    match action {
        NFTAction::Mint {
            transaction_id,
            token_metadata,
        } => {
            nft.check_constraints();
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::mint(nft, token_metadata))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::Burn {
            transaction_id,
            token_id,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(NFTCore::burn(nft, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::Transfer {
            transaction_id,
            to,
            token_id,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(NFTCore::transfer(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        }
        NFTAction::TransferPayout {
            transaction_id,
            to,
            token_id,
            amount,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::TransferPayout(NFTCore::transfer_payout(nft, &to, token_id, amount))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::TransferPayout`");
        }
        NFTAction::NFTPayout { owner, amount } => {
            msg::reply(
                NFTEvent::NFTPayout(NFTCore::nft_payout(nft, &owner, amount)),
                0,
            )
            .expect("Error during replying with `NFTEvent::NFTPayout`");
        }
        NFTAction::Approve {
            transaction_id,
            to,
            token_id,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Approval(NFTCore::approve(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        }
        NFTAction::Owner { token_id } => {
            msg::reply(
                NFTEvent::Owner {
                    owner: NFTCore::owner_of(nft, token_id),
                    token_id,
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::Owner`");
        }
        NFTAction::IsApproved { to, token_id } => {
            msg::reply(
                NFTEvent::IsApproved {
                    to,
                    token_id,
                    approved: NFTCore::is_approved_to(nft, &to, token_id),
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::IsApproved`");
        }
        NFTAction::DelegatedApprove {
            transaction_id,
            message,
            signature,
        } => {
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Approval(NFTCore::delegated_approve(nft, message, signature))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        }
        NFTAction::Clear { transaction_hash } => nft.clear(transaction_hash),
        NFTAction::AddMinter {
            transaction_id,
            minter_id,
        } => {
            nft.check_constraints();
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    nft.constraints.authorized_minters.push(minter_id);
                    NFTEvent::MinterAdded { minter_id }
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        },
        NFTAction::NFTData(token_id) => {
            let caller = msg::source();
            if caller != nft.main_contract && caller != nft.owner {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            
            if !NFTCore::is_approved_to(nft, &caller, token_id) {
                msg::reply(NFTEvent::MainContractIsNotApproved, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            
            //msg::reply(NFTEvent::NFTData(None), 0)
            //    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
            
            msg::reply(NFTEvent::NFTData(token_metadata_helper(&token_id, nft)), 0)
                .expect("Error during replying with 'NFTEvent::NFTData'");
        },
        NFTAction::NFTDataFromUsers(users) => {
            let caller = msg::source();
            if caller != nft.main_contract && caller != nft.owner  {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            let response: Vec<(ActorId, Vec<TokenMetadata>)> = users
                .into_iter()
                .map(|user| {
                    let mut  tokens_metadata = Vec::new();
                    if let Some((_owner, token_ids)) = nft
                        .token
                        .tokens_for_owner
                        .iter()
                        .find(|(id, _tokens)| user.eq(id))
                    {
                        for token_id in token_ids {
                            tokens_metadata.push(token_metadata_helper(token_id, nft).unwrap());
                        }
                    }
                    
                    (user, tokens_metadata)
                })
                .collect();
            
            msg::reply(NFTEvent::AllNFTInformation(response), 0)
                .expect("Error during replying with 'NFTEvent::AllNFTInformation'");
        },
        NFTAction::SetMainContract(contract_address) => {
            if msg::source() != nft.owner {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            nft.main_contract = contract_address;
            
            msg::reply(NFTEvent::MainContractSet, 0)
                .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
        },
        NFTAction::MintNFTsTo {
            to,
            nfts
        } => {
            let caller = msg::source();
            if caller != nft.owner && caller != nft.main_contract {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            let mut tokens_id = Vec::new();
            nfts
                .into_iter()
                .for_each(|token_metadata| {
                    let NFTTransfer { token_id, .. } = MyNFTCore::mint(nft, token_metadata);
                    tokens_id.push(token_id);
                });
            tokens_id
                .into_iter()
                .for_each(|token_id| {
                    NFTCore::transfer(nft, &to, token_id); 
                });
            
            msg::reply(
                NFTEvent::NFTsMinted,
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        },
        NFTAction::BurnAllNFTS => {
            let caller = msg::source();
            if caller != nft.owner && caller != nft.main_contract {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            
            nft.transactions.clear();
            nft.token.owner_by_id.clear();
            nft.token.token_approvals.clear();
            nft.token.token_metadata_by_id.clear();
            nft.token.tokens_for_owner.clear();
            
            msg::reply(NFTEvent::AllBurned, 0)
                .expect("Error during replying with 'NFTEvent::AllBurned'");
        },
        NFTAction::DeleteContract => {
            let caller = msg::source();
            if caller != nft.owner {
                msg::reply(NFTEvent::ActionOnlyForMainContract, 0)
                    .expect("Error during replying with 'NFTEvent::ActionOnlyForMainContract'");
                return;
            }
            
            exec::exit(caller);
        }
    };
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer;
}

impl MyNFTCore for Contract {
    fn mint(&mut self, token_metadata: TokenMetadata) -> NFTTransfer {
        let transfer = NFTCore::mint(self, &msg::source(), self.token_id, Some(token_metadata));
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
}

impl Contract {
    fn process_transaction(
        &mut self,
        transaction_id: u64,
        action: impl FnOnce(&mut Contract) -> NFTEvent,
    ) -> NFTEvent {
        let transaction_hash = get_hash(&msg::source(), transaction_id);

        if let Some(nft_event) = self.transactions.get(&transaction_hash) {
            nft_event.clone()
        } else {
            let nft_event = action(self);

            self.transactions
                .insert(transaction_hash, nft_event.clone());

            nft_event
        }
    }

    fn clear(&mut self, transaction_hash: H256) {
        assert_eq!(
            msg::source(),
            exec::program_id(),
            "Not allowed to clear transactions"
        );
        self.transactions.remove(&transaction_hash);
    }

    fn check_constraints(&self) {
        if let Some(max_mint_count) = self.constraints.max_mint_count {
            if max_mint_count <= self.token.token_metadata_by_id.len() as u32 {
                panic!(
                    "Mint impossible because max minting count {} limit exceeded",
                    max_mint_count
                );
            }
        }

        let current_minter = msg::source();
        let is_authorized_minter = self
            .constraints
            .authorized_minters
            .iter()
            .any(|authorized_minter| authorized_minter.eq(&current_minter));

        if !is_authorized_minter {
            panic!(
                "Current minter {:?} is not authorized at initialization",
                current_minter
            );
        }
    }
}

fn static_mut_state() -> &'static Contract {
    unsafe { CONTRACT.get_or_insert(Default::default()) }
}

fn common_state() -> IoNFT {
    static_mut_state().into()
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state())
        .expect("Failed to encode or reply with `<NFTMetadata as Metadata>::State` from `state()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
}

impl From<&Contract> for IoNFT {
    fn from(value: &Contract) -> Self {
        let Contract {
            token,
            token_id,
            owner,
            transactions,
            ..
        } = value;

        let transactions = transactions
            .iter()
            .map(|(key, event)| (*key, event.clone()))
            .collect();
        Self {
            token: token.into(),
            token_id: *token_id,
            owner: *owner,
            transactions,
        }
    }
}

impl From<&Contract> for State {
    fn from(value: &Contract) -> Self {
        let Contract {
            token,
            token_id,
            owner,
            transactions,
            collection,
            constraints,
            main_contract: _
        } = value;

        let owners = token
            .owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*actor_id, *hash))
            .collect();

        let transactions = transactions
            .iter()
            .map(|(hash, event)| (*hash, event.clone()))
            .collect();

        let token_metadata_by_id = token
            .token_metadata_by_id
            .iter()
            .map(|(id, metadata)| {
                let metadata = metadata.as_ref().unwrap();
                let nft = Nft {
                    owner: token.owner_by_id[id],
                    name: metadata.name.clone(),
                    description: metadata.description.clone(),
                    media_url: metadata.media.clone(),
                    attrib_url: metadata.reference.clone(),
                };
                (*id, nft)
            })
            .collect();

        Self {
            tokens: token_metadata_by_id,
            collection: collection.clone(),
            nonce: *token_id,
            owners,
            owner: *owner,
            transactions,
            constraints: constraints.clone(),
        }
    }
}

fn token_metadata_helper(token_id: &TokenId, state: &Contract) -> Option<TokenMetadata> {
    match state.token.token_metadata_by_id.get(token_id) {
        Some(data) => {
            data.clone()
        },
        None =>None
    }
    /*
    let mut token_metadata = TokenMetadata::default();
    if let Some((_token_id, Some(metadata))) = state
        .token
        .token_metadata_by_id
        .iter()
        .find(|(id, _metadata)| token_id.eq(id))
    {
        token_metadata.name = metadata.name.clone();
        token_metadata.description = metadata.description.clone();
        token_metadata.media = metadata.media.clone();
        token_metadata.reference = metadata.reference.clone();
    } else {
        return None;
    }
    
    Some(token_metadata)
    */
}