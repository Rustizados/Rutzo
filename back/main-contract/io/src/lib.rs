#![no_std]
use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::{TokenId, TokenMetadata},
};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, BTreeMap, msg, exec};
use primitive_types::{ H256 };

pub struct ProgramMetadata;

pub type NftContractId = ActorId;
pub type UserId = ActorId;
pub type MatchId = ActorId;
pub type InGame = bool;

impl Metadata for ProgramMetadata {
    type Init = In<InitContractData>;
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<ContractState>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum RutzoAction {
    PlayGame {
        token_id: TokenId,
        power: String
    },
    MintCard {
        token_id: u8
    },
    SetNFTAddress(ActorId),
    Register,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum RutzoEvent {
    Minted(TokenId),
    NFTContractSaved,
    ErrorCallingNFTContract,
    UserIsNotApproved(ActorId),
    UserIsNotTheOwner(ActorId),
    Approved(ActorId),
    ErrorBuying(String),
    NFTWithIdNotExists(u8),
    AccountAlreadyExist(UserId),
    AccountNotExists(UserId),
    AccountAlreadyInMatch(UserId),
    QueryNotAllowed(String),
    RegisterSucces,
    LoginSucces,
    UserInMatch(MatchId),
    ErrorInJoiningMatch,
    PurchaseSucces,
    ReplySuccess,
    NewPlayer(ActorId),
    MatchFinished,
    MatchCreated,
    MaxMintsReached(ActorId)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum GameState {
    WaitigForPlayer,
    GameInProgress
}

#[derive(Encode, Decode, TypeInfo)]
pub struct ContractState {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    pub users: Vec<(UserId, InGame)>,
    pub games: Vec<MatchInformation>,
    pub tokens_metadata_default: Vec<(u8, TokenMetadata)>,
    pub default_tokens_minted_by_id: Vec<(UserId, Vec<u8>)>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: u64
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitContractData {
    pub nft_contract: Option<ActorId>,
    pub tokens_metadata_default: Vec<TokenMetadata>,
}

#[derive(Encode, Decode, TypeInfo, Default)]
pub struct UserData {
    pub user_id: ActorId,
    pub chosen_nft: TokenId,
    pub power: u8
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
pub enum MatchState {
    Finished {
        winner: ActorId,
        loser: ActorId
    },
    #[default]
    InProgress,
    NotExists
}

#[derive(Encode, Decode, TypeInfo)]
pub struct MatchInformation {
    pub user_1: UserData,
    pub user_2: Option<UserData>,
    pub match_state: MatchState,
}

#[derive(Encode, Decode, TypeInfo, Default)]
pub struct ContractData {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    pub users: BTreeMap<UserId, InGame>,
    pub games: Vec<MatchInformation>,
    pub tokens_metadata_default: BTreeMap<u8, TokenMetadata>,
    pub default_tokens_minted_by_id: BTreeMap<UserId, Vec<u8>>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: u64
}

impl ContractData {
    pub async fn mint_card(&mut self, token_id: u8) -> RutzoEvent {
        let user_id = msg::source();
        
        if !self.is_register(&user_id) {
            return RutzoEvent::AccountNotExists(user_id);
        } 
        
        if self.default_tokens_minted_by_id.get(&user_id).unwrap().len() == 3 {
            return RutzoEvent::MaxMintsReached(user_id);    
        }
        
        let token_metadata = self.tokens_metadata_default
            .get(&token_id);
           
        let token_metadata = match token_metadata {
            Some(token_data) => token_data.clone(),
            None => return RutzoEvent::NFTWithIdNotExists(token_id)
        };
        
        let answer: NFTEvent = msg::send_for_reply_as::<NFTAction, NFTEvent>(
            self.nft_contract.unwrap(), 
            NFTAction::Mint { 
                transaction_id: self.transaction_id, 
                token_metadata
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::MintFromMainContract' to a match contract")
        .await
        .expect("Unable to decode NFTEvent");
        
        let NFTEvent::Transfer(transfer_data) = answer else {
            return RutzoEvent::ErrorCallingNFTContract; 
        };
        
        self.transaction_id += 1;
        
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            self.nft_contract.unwrap(), 
            NFTAction::Transfer { 
                transaction_id: self.transaction_id, 
                to: user_id, 
                token_id: transfer_data.token_id 
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::MintFromMainContract' to a match contract")
        .await
        .expect("Unable to decode NFTEvent");
        
        self.default_tokens_minted_by_id
            .entry(user_id)
            .and_modify(|minted| minted.push(token_id))
            .or_insert(Vec::new());
        
        self.transaction_id += 1;
        
        RutzoEvent::Minted(TokenId::default())//transfer_data.token_id)
    }
    
    pub fn add_minter(&mut self, user_id: ActorId) -> RutzoEvent {
        let caller = msg::source();
        if caller != self.owner {
            return RutzoEvent::UserIsNotTheOwner(caller);
        }
        
        self.approved_minters.push(user_id);
        
        RutzoEvent::Approved(user_id)
    }
    
    pub fn register_user(&mut self, user_id: UserId) -> RutzoEvent {
        if self.is_register(&user_id) {
            return RutzoEvent::AccountAlreadyExist(user_id);
        }
      
        self.users.insert(user_id, false);        
        self.default_tokens_minted_by_id.insert(user_id, Vec::new());
        RutzoEvent::RegisterSucces
    }
    
    pub fn is_register(&self, user_id: &UserId) ->  bool {
        self.users.contains_key(user_id)
    }
    
    pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId, power: u8) -> RutzoEvent {
        if !self.is_register(&user_id) {
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        if *self.users.get(&user_id).unwrap() {
            return RutzoEvent::AccountAlreadyInMatch(user_id);
        }
        
        for game in self.games.iter_mut() {
            if let MatchState::InProgress = game.match_state {
                game.user_2 = Some(UserData {
                    user_id,
                    chosen_nft: token_id,
                    power
                });
                let user1_power = game.user_1.power;
                
                let (to, token_id) = if user1_power > power {
                    game.match_state = MatchState::Finished { 
                        winner: game.user_1.user_id, 
                        loser: user_id 
                    };
                    
                    let token_id_user2 = game.user_2.as_ref().unwrap().chosen_nft.clone();
                    let user1_id = game.user_1.user_id;
                    
                    (user1_id, token_id_user2)
                } else {
                    game.match_state = MatchState::Finished { 
                        winner: user_id, 
                        loser:  game.user_1.user_id
                    };
                    
                    let token_id_user1 = game.user_1.chosen_nft;
                    let user2_id = game.user_2.as_ref().unwrap().user_id.clone();
                    
                    (user2_id, token_id_user1)
                };
                
                msg::send_for_reply_as::<NFTAction, NFTEvent>(
                    self.nft_contract.unwrap(), 
                    NFTAction::Transfer { 
                        transaction_id: self.transaction_id, 
                        to, 
                        token_id 
                    }, 
                    0, 
                    0
                )
                .expect("Error in sending a message 'NFTAction::MintFromMainContract' to a match contract")
                .await
                .expect("Unable to decode NFTEvent");
                
                self.transaction_id += 1;
                return RutzoEvent::MatchFinished;
            }
        }
        
        self.games.push(MatchInformation {
            user_1: UserData {
                user_id,
                chosen_nft: token_id,
                power
            },
            user_2: None,
            match_state: MatchState::default()
        });
        
        RutzoEvent::MatchCreated
    }
}

impl From<ContractData> for ContractState {
    fn from(value: ContractData) -> Self {
        let ContractData {
            owner,
            nft_contract,
            users,
            games,
            tokens_metadata_default,
            default_tokens_minted_by_id,
            approved_minters,
            transaction_id
        } = value;
        
        let users = users
            .iter()
            .map(|(user_id, in_game)| (*user_id, in_game.clone()))
            .collect();
        let tokens_metadata_default = tokens_metadata_default
            .iter()
            .map(|(token_id, token_metadata)| (*token_id, token_metadata.clone()))
            .collect();
        let default_tokens_minted_by_id = default_tokens_minted_by_id
            .iter()
            .map(|(user_id, minted)| (*user_id, minted.clone()))
            .collect();
        
        Self {
            owner,
            nft_contract,
            users,
            games,
            transaction_id,
            tokens_metadata_default,
            default_tokens_minted_by_id,
            approved_minters
        }
    }
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
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
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
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
}

/*
 [
            [
                "0xeaee0180c37581cadedafb9eb59d8ecdf68be2f93bcb021124963f7ba7f7b90d",
                false
            ]
        ],
        "games": [],
        "tokensMetadataDefault": [
            {
                "name": "ixchel",
                "description": "wind",
                "media": "https://home.rutzo.studio/NFT/ixchel_wind.jpg",
                "reference": "33"
            },
            {
                "name": "Death City Earth",
                "description": "rock",
                "media": "https://home.rutzo.studio/NFT/death_city_earth.jpg",
                "reference": "20"
            },
            {
                "name": "Chile",
                "description": "fire",
                "media": "https://home.rutzo.studio/NFT/chile_fire.jpg",
                "reference": "55"
            },
            {
                "name": "Chinampa",
                "description": "water",
                "media": "https://home.rutzo.studio/NFT/chinampa_water.jpg",
                "reference": "50"
            },
            {
                "name": "ehecatl",
                "description": "wind",
                "media": "https://home.rutzo.studio/NFT/ehecatl_wind.jpg",
                "reference": "75"
            },
            {
                "name": "huitzilopochtli",
                "description": "rock",
                "media": "https://home.rutzo.studio/NFT/huitzilopochtli_earth.jpg",
                "reference": "43"
            }
        ]
*/