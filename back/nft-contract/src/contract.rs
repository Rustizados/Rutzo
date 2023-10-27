use program_io::*;

use gear_lib_derive::{
    NFTCore, 
    NFTMetaState, 
    NFTStateKeeper
};

use gear_lib::non_fungible_token::{
    io::{ NFTTransfer, NFTApproval },
    nft_core::*,
    state::*, 
    token::*
};
use gstd::{
    HashMap, 
    msg,
    prelude::*,
    ActorId,
    exec,
    debug
};
use hashbrown::HashSet;
use primitive_types::{H256, U256};

#[derive(Debug, Default, NFTStateKeeper, NFTCore, NFTMetaState)]
pub struct Contract {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub main_contract: Option<ActorId>,
    pub base_image: String,
    pub nfts: HashMap<TokenId, RutzoNft>,
    pub nft_available: HashSet<TokenId>,
    pub given_default_nfts: Vec<ActorId>,
    pub default_nfts: Vec<(TokenMetadata, RutzoNft)>,
    pub transactions: HashMap<H256, NFTEvent>,
    pub config: Option<Config>, 
    pub marketplace_id: Option<ActorId>
}

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
unsafe extern fn init() {
    let config: InitNFT = msg::load()
        .expect("Unable to decode InitNFT");
    if config.royalties.is_some() {
        config.royalties.as_ref().expect("Unable to g").validate();
    }
    let mut nft = Contract {
        token: NFTState {
            name: config.name,
            royalties: config.royalties,
            symbol: config.symbol,
            base_uri: config.base_uri,
            ..Default::default()
        },
        owner: msg::source(),
        main_contract: config.main_contract,
        base_image: config.base_image,
        //nfts: HashMap::from_iter(config.nfts.into_iter()), Default
        default_nfts: config.default_nfts,
        config: None,
        marketplace_id: config.marketplace_actorid,
        ..Default::default()
    };
    // Mint all NFTs data with their TokenMetadata
    // to the owner, for future sales.
    
    config.nfts
        .into_iter()
        .for_each(|nft_data| {
            if let Some(marketplace_actorid) = config.marketplace_actorid {
                MyNFTCore::mint_with_approval(&mut nft, nft_data.0, nft_data.1, &marketplace_actorid);
            } else {
                MyNFTCore::mint(&mut nft, nft_data.0, nft_data.1);
            }
        });
    
    CONTRACT = Some(nft);
}

#[no_mangle]
unsafe extern fn handle() {
    let action: NFTAction = msg::load().expect("Could not load NFTAction");
    let nft = CONTRACT.get_or_insert(Default::default());
    if !nft.main_contract.is_some() {
        if let NFTAction::SetMainContract { .. } = action {
            
        } else {
            msg::reply(NFTEvent::NoMainContractInActualContract, 0)
                .expect("Error during replying with `NFTEvent::NoMainContractInActualContract`");
            return;
        }
        
    }
    match action {
        NFTAction::SetMainContract {  // -----------------------------------------------------
            main_contract_id  
        } => {
            if nft.owner != msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserIsNotTheOwner`");
                return;
            }
            nft.main_contract = Some(main_contract_id);
            MyNFTCore::approve_actor_id_to_availables_nft(nft, main_contract_id);
            msg::reply(NFTEvent::MainContractEstablished, 0)
                    .expect("Error during replying with `NFTEvent::MainContractEstablished`");
        },
        NFTAction::ApproveMarketplace {  // -----------------------------------------------------
            transaction_id,
            marketplace_id
        } => {
            if nft.owner != msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserIsNotTheOwner`");
                return;
            }
            
            nft.marketplace_id = Some(marketplace_id);
            
            nft.process_transaction(transaction_id, |nft| {
                //NFTEvent::Approval(MyNFTCore::approve_marketplace(nft, marketplace_id))
                NFTEvent::Approval(MyNFTCore::approve_actor_id_to_availables_nft(nft, marketplace_id))
            });
            
            msg::reply(
                NFTEvent::MarketplaceApproved,
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        },
        NFTAction::MintDefault {   // ----------------------------------------------
            transaction_id,
            to,
        } => {
            if !nft.is_authorized(&msg::source()) && nft.main_contract.unwrap() != msg::source() {
                msg::reply(NFTEvent::UserNotAllowedToMint, 0)
                    .expect("Error during replying with `NFTEvent::UserNotAllowedToMint`");
                return;
            }
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::mint_default(nft, to))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        },
        NFTAction::Mint {
            transaction_id,
            nft_data,
            token_metadata,
        } => {
            if !nft.is_authorized(&msg::source()) && nft.owner != msg::source() {
                msg::reply(NFTEvent::UserNotAllowedToMint, 0)
                    .expect("Error during replying with `NFTEvent::UserNotAllowedToMint`");
                return;
            }
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::mint(nft, token_metadata, nft_data))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        },
        NFTAction::Burn {
            transaction_id,
            token_id,
        } => {
            /*
            Gear NFT Api already has this conditionals
            if !nft.is_authorized(&msg::source()) || NFTCore::owner_of(nft, token_id) == msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserNotAllowedToMint`");
                return;
            }
            */
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::burn(nft, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
        },
        NFTAction::Transfer {
            transaction_id,
            to,
            token_id,
        } => {
            /*
            Gear NFT Api already has this conditionals
            if !nft.is_authorized(&msg::source()) && NFTCore::owner_of(nft, token_id) != msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserNotAllowedToMint`");
                return;
            }
            */
            
            msg::reply(
                NFTEvent::Transfer(MyNFTCore::transfer(nft, &to, token_id)),
                0
            ).expect("Error during replying with `NFTEvent::Transfer`");
            /*
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Transfer(MyNFTCore::transfer(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Transfer`");
            */
        },
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
        },
        NFTAction::NFTPayout { owner, amount } => {
            msg::reply(
                NFTEvent::NFTPayout(NFTCore::nft_payout(nft, &owner, amount)),
                0,
            )
            .expect("Error during replying with `NFTEvent::NFTPayout`");
        },
        NFTAction::Approve {
            transaction_id,
            to,
            token_id,
        } => {
            if NFTCore::owner_of(nft, token_id) != msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserIsNotTheOwner`");
                return;
            }
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    NFTEvent::Approval(NFTCore::approve(nft, &to, token_id))
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        },
        NFTAction::Owner { token_id } => {
            msg::reply(
                NFTEvent::Owner {
                    owner: NFTCore::owner_of(nft, token_id),
                    token_id,
                },
                0,
            )
            .expect("Error during replying with `NFTEvent::Owner`");
        },
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
        },
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
        },
        NFTAction::Clear { transaction_hash } => nft.clear(transaction_hash),
        NFTAction::AddMinter {
            transaction_id,
            minter_id,
        } => {
            if nft.owner != msg::source() {
                msg::reply(NFTEvent::UserIsNotTheOwner, 0)
                    .expect("Error during replying with `NFTEvent::UserIsNotTheOwner`");
                return;
            }
            if let None = nft.config {
                nft.config = Some(Config {
                    max_mint_count: Some(300),
                    authorized_minters: Vec::new()
                });
            }
            msg::reply(
                nft.process_transaction(transaction_id, |nft| {
                    nft.config.as_mut().unwrap().authorized_minters.push(minter_id);
                    NFTEvent::MinterAdded { minter_id }
                }),
                0,
            )
            .expect("Error during replying with `NFTEvent::Approval`");
        },
        NFTAction::NFTDataFromTokenId {
            tokens_id
        } => {
            
        }
    };
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0)
        .expect("Failed to encode or reply with `State` from `state()`");
}

pub fn get_hash(account: &ActorId, transaction_id: u64) -> H256 {
    let account: [u8; 32] = (*account).into();
    let transaction_id = transaction_id.to_be_bytes();
    sp_core_hashing::blake2_256(&[account.as_slice(), transaction_id.as_slice()].concat()).into()
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
    
    fn is_authorized(&self, user_id: &ActorId) -> bool {
        if self.config.is_some() && self.check_config(user_id).is_ok() {
            return true;
        }
        if self.owner == *user_id {
            return true;
        }
        false
    }

    fn check_config(&self, current_minter: &ActorId) -> Result<(), String>{
        if !self.config.is_some() {
            return Err(String::from("No additional minters"));
        }
        if let Some(max_mint_count) = self.config.as_ref().unwrap().max_mint_count {
            if max_mint_count <= self.token.token_metadata_by_id.len() as u32 {
                return Err(
                    format!("Mint impossible because max minting count {} limit exceeded", max_mint_count)
                );
            }
        }

        //let current_minter = msg::source();
        let is_authorized_minter = self
            .config
            .as_ref()
            .unwrap()
            .authorized_minters
            .iter()
            .any(|authorized_minter| authorized_minter.eq(&current_minter));

        if !is_authorized_minter {
            return Err(
                format!("Current minter {:?} is not authorized at initialization", current_minter)
            );
        }
        Ok(())
    }
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, token_metadata: TokenMetadata, nft: RutzoNft) -> NFTTransfer;
    fn burn(&mut self, token_id: TokenId) -> NFTTransfer;
    fn transfer(&mut self, to: &ActorId, token_id: TokenId) -> NFTTransfer;
    fn mint_default(&mut self, user_id: ActorId) -> NFTTransfer;
    fn mint_with_approval(&mut self, token_metadata: TokenMetadata, nft: RutzoNft, approve: &ActorId) -> NFTTransfer;
    fn approve_actor_id_to_availables_nft(&mut self, actor_id: ActorId) -> NFTApproval;
    //fn approve_marketplace(&mut self, marketplace_id: ActorId) -> NFTApproval;
    //fn approve_main_contract(&mut self, main_contract_id: ActorId) -> NFTApproval;
}

impl MyNFTCore for Contract {
    fn mint(&mut self, token_metadata: TokenMetadata, nft: RutzoNft) -> NFTTransfer {
        // Mint new token to the owner contract, this for selling the NFT
        // if the NFT is sold, the contract transfer the NFT to the new owner
        let contract_id = self.owner.clone();
        let transfer = NFTCore::mint(self, &contract_id, self.token_id, Some(token_metadata));
        if let Some(marketplace_id) = self.marketplace_id {
            NFTCore::approve(self, &marketplace_id, self.token_id);
        }
        if let Some(main_contract_id) = self.main_contract {
            NFTCore::approve(self, &main_contract_id, self.token_id);
        }
        
        self.nft_available.insert(self.token_id);
        self.nfts.insert(self.token_id, nft);
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
    
    fn mint_default(&mut self, user_id: ActorId) -> NFTTransfer {
        // Mint default token to the specific account
        let mut last_transfer = Default::default();
        self.default_nfts
            .clone()
            .iter()
            .for_each(|default_nft| {
                NFTCore::mint(self, &self.main_contract.unwrap().clone(), self.token_id, Some((default_nft.0).clone()));
                NFTCore::approve(self, &self.main_contract.unwrap().clone(), self.token_id);
                NFTCore::approve(self, &self.owner.clone(),  self.token_id);
                last_transfer = NFTCore::transfer(self, &user_id, self.token_id);
                self.nfts.insert(self.token_id, (default_nft.1).clone());
                self.token_id = self.token_id.saturating_add(U256::one());
            });
        last_transfer
    }
    
    fn mint_with_approval(&mut self, token_metadata: TokenMetadata, nft: RutzoNft, approve: &ActorId) -> NFTTransfer {
        // This is only for owner, to approve marketplace contract to 
        // sale the NFT
        let contract_id = self.owner.clone();
        let transfer = NFTCore::mint(self, &contract_id, self.token_id, Some(token_metadata));
        NFTCore::approve(self, approve, self.token_id);
        self.nfts.insert(self.token_id, nft);
        self.nft_available.insert(self.token_id);
        self.token_id = self.token_id.saturating_add(U256::one());
        transfer
    }
    
    fn burn(&mut self, token_id: TokenId) -> NFTTransfer {
        let transfer = NFTCore::burn(self, token_id);
        self.nfts.remove(&token_id);
        self.nft_available.remove(&token_id);
        transfer
    }
    
    fn transfer(&mut self, to: &ActorId, token_id: TokenId) -> NFTTransfer {
        let source_id = msg::source();
        if source_id == self.marketplace_id.unwrap() {
            NFTCore::transfer(self, &self.marketplace_id.unwrap().clone(), token_id);
            NFTCore::revoke_approval(self, &self.marketplace_id.unwrap().clone(), token_id);
            NFTCore::approve(self, &self.owner.clone(), token_id);
        }
        NFTCore::transfer(self, to, token_id)
    }
    
    fn approve_actor_id_to_availables_nft(&mut self, actor_id: ActorId) -> NFTApproval {
        // Only for initialize the mut variable 
        let mut last_approval = NFTApproval::default();
        self.nft_available
            .clone()
            .iter()
            .for_each(|nft_id| {
                last_approval = NFTCore::approve(self, &actor_id, *nft_id);
            });
        
        last_approval
    }

}

impl From<Contract> for State {
    fn from(value: Contract) -> Self {
        let Contract {
            token,
            token_id,
            owner,
            base_image,
            nfts,
            nft_available,
            given_default_nfts,
            default_nfts,
            transactions,
            config,
            marketplace_id,
            main_contract      
        } = value;
        
        let nfts = nfts
            .iter()
            .map(|(token_id, item_id)| (*token_id, item_id.clone()))
            .collect();
        let nft_available = nft_available
            .iter()
            .cloned()
            .collect();
        let transactions = transactions
            .iter()
            .map(|(hash, event)| (*hash, event.clone()))
            .collect();
        
        Self {
            token: (&token).into(),
            token_id,
            owner,
            base_image,
            nfts,
            nft_available,
            given_default_nfts: given_default_nfts.clone(),
            default_nfts: default_nfts.clone(),
            transactions,
            config,
            marketplace_id,
            main_contract
        }
    }
}



        /*
        let owners = token
            .owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*actor_id, *hash))
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
            collection,
            nonce: token_id,
            owners,
            owner,
            transactions,
            config,
        }
        */


/*
impl From<Contract> for IoNft {
    fn from(value: Contract) -> Self {
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
            token: (&token).into(),
            token_id,
            owner,
            transactions,
        }
    }
}
*/





// Old code for transfer NFT
        // Fix to does not remove approval from main contract and owner
        // for possibly exploits,etc.
        /*
        let caller = msg::source();
        let nft_owner = NFTCore::owner_of(self, token_id);
        let is_approved = NFTCore::is_approved_to(self, &caller, token_id);
        let mut from_owner = false;
        */
        
        /*
        if caller != nft_owner || !is_approved {
            panic!("User is not the owner or approved of the NFT");
        }
        */
        
        /*
        if nft_owner != self.owner {
            panic!("Nombre");
        }
        */
        
        
        
        /*
        if  nft_owner == self.owner && caller == self.marketplace_id.unwrap()  {
            self.nft_available.remove(&token_id);
            //NFTCore::revoke_approval(self, &self.marketplace_id.unwrap().clone(), token_id);
            //NFTCore::approve(self, &self.main_contract.unwrap().clone(), token_id);
            //NFTCore::approve(self, &self.owner.clone(), token_id);
            from_owner = true;
        } else if nft_owner == self.owner && caller != self.marketplace_id.unwrap() {
           // panic!("Warning, owner cannot transfer tokens to player, only the marketplace contract");
        }
        */
        
        
        
            /*
    fn approve_marketplace(&mut self, marketplace_id: ActorId) -> NFTApproval {
        // Only for initialize the mut variable 
        let mut last_approval = NFTApproval::default();
        self.nft_available
            .clone()
            .iter()
            .for_each(|nft_id| {
                last_approval = NFTCore::approve(self, &marketplace_id, *nft_id);
            });
        
        last_approval
    }
    
    fn approve_main_contract(&mut self, main_contract_id: ActorId) -> NFTApproval {
        // Only for initialize the mut variable 
        let mut last_approval = NFTApproval::default();
        self.nft_available
            .clone()
            .iter()
            .for_each(|nft_id| {
                last_approval = NFTCore::approve(self, &main_contract_id, *nft_id);
            });
        
        last_approval
    }
    */