#![no_std]
use gstd::{prelude::*, ActorId, BTreeMap, exec, CodeId, prog::ProgramGenerator, msg};
use gmeta::{Metadata, InOut, Out, In};
use gear_lib::non_fungible_token::token::TokenId;


use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::*,
};

use primitive_types::{ H256 };

pub struct ProgramMetadata;

pub type NftContractId = ActorId;
pub type UserId = ActorId;
pub type MatchId = ActorId;
pub type InGame = bool;

pub const MATCH_CODEID: &str = "aa7b8462984cba48bab6bce9b599caea61d914cbcb9682d654e4709b2abbcd3a";
pub const GAS_FOR_CREATION_MATCH: u64 = 11_000_000_000;

impl Metadata for ProgramMetadata {
    type Init = In<InitContractData>;
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<ContractState>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractAction {
    NewUser(ActorId),
    MatchResult(MatchContractResults),
    AcceptedUser,
    QueryNotAllowed(String),
    DeleteContract,    
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractEvent {
    UserAddedToMatch,
    MatchBegin,
    MatchResult(MatchContractResults),
    ContractDeleted,
    QueryNotAllowed(String),
    ErrorAddingUser((ActorId, String))
}

//  State: Beta   -----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractData {
    pub first_user: Option<ActorId>,
    pub second_user: Option<ActorId>,
    pub mainContractId: ActorId
}

// State: Beta ----------------------------
#[derive(Encode, Decode, TypeInfo)]
pub struct MatchContractResults {
    pub data: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum RutzoAction {
    Login,
    Register,
    PlayGame,
    QuitGame,
    GetCards,
    BuyCard,
    EndGame,
    TransferCard {
        from: ActorId,
        to: ActorId,
        matchId: ActorId
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum RutzoEvent {
    ErrorBuying(String),
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
    NewPlayerWithMainId(NewPlayerWithIdData)
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewPlayerWithIdData {
    pub main_id: ActorId,
    pub new_player: ActorId
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct GameData {
    pub match_id: ActorId,
    pub actual_state: GameState
}

#[derive(Decode, Encode, TypeInfo)]
pub struct UserData {
    pub user_id: ActorId,
    pub in_game: bool
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum GameState {
    WaitigForPlayer,
    GameInProgress
}

#[derive(Encode, Decode, TypeInfo)]
pub struct ContractState {
    pub owner: ActorId,
    pub match_contract: String,
    pub nft_contract: NftContractId,
    pub users: Vec<(UserId, InGame)>,
    pub games: Vec<(MatchId, GameState)>,
    pub transaction_id: u64
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitContractData {
    pub match_contract: String,
    pub nft_contract: ActorId,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct ContractData {
    pub owner: ActorId,
    pub match_contract: String,
    pub nft_contract: NftContractId,
    pub users: BTreeMap<UserId, InGame>,
    pub games: BTreeMap<MatchId, GameState>,
    pub transaction_id: u64
}

impl ContractData {
    pub async fn register_user(&mut self, user_id: UserId) -> RutzoEvent {
        if self.is_register(&user_id) {
            return RutzoEvent::AccountAlreadyExist(user_id);
        }
        
        msg::send_for_reply_as::<NFTAction, NFTEvent>(self.nft_contract, NFTAction::MintDefault { 
            transaction_id: self.transaction_id, 
            to: user_id
        }, 0, 0)
        .expect("Error in sending a message 'NFTAction::MintDefault' to a match contract")
        .await
        .expect("Unable to decode NFTEvent");
        
        self.users.insert(user_id, false);
        
        RutzoEvent::RegisterSucces
    }
    
    pub fn is_register(&self, user_id: &UserId) ->  bool {
        self.users.contains_key(user_id)
    }
    
    pub async fn join_player_to_game(&mut self, user_id: ActorId) -> RutzoEvent {
        if self.is_register(&user_id) {
            return RutzoEvent::AccountAlreadyExist(user_id);
        }
        
        let user_in_game = match self.users.get(&user_id) {
            Some(user_data) => *user_data,
            None => return RutzoEvent::AccountAlreadyExist(user_id)
        };
        
        if user_in_game {
            return RutzoEvent::AccountAlreadyInMatch(user_id);
        }      
        
        let match_id = match self.find_match_waiting_player() {
            Some(match_id) => match_id,
            None => {
                let match_id = self.create_match().await;
                self.games.insert(match_id, GameState::WaitigForPlayer);
                match_id
            }
        };
        
        let match_contract_message = msg::send_for_reply_as::<MatchContractAction, MatchContractEvent>(
            match_id, 
            MatchContractAction::NewUser(user_id), 
            0, 
            0
        )
        .expect("Error in sending a message 'MatchContractAction::NewUser(ActorId)' to a match contract")
        .await;
        
        match match_contract_message {
            Ok(match_action) => {
                match match_action {
                    MatchContractEvent::MatchBegin => {
                        self.games
                            .entry(match_id)
                            .and_modify(|game_state| *game_state = GameState::GameInProgress);
                    },
                    MatchContractEvent::ErrorAddingUser(_) => {
                        return RutzoEvent::ErrorInJoiningMatch;
                    }
                    _ => {}
                }
            },
            Err(_) => return RutzoEvent::ErrorInJoiningMatch
        }
        
        self.users
            .entry(user_id)
            .and_modify(|in_game| *in_game = true);
        
        RutzoEvent::UserInMatch(ActorId::default())
    }
    
    pub fn find_match_waiting_player(&self) -> Option<ActorId> {
        let match_data = self.games
            .iter()
            .find(|&(_, game_state)| {
                if let GameState::WaitigForPlayer = *game_state {
                    true
                } else {
                    false
                }
            });
        match match_data {
            Some((match_id, _)) => Some(match_id.clone()),
            None => None
        }
    }
    
    pub async fn create_match(&self) -> ActorId {
        let submitted_code: CodeId = hex_literal::hex!("abf3746e72a6e8740bd9e12b879fbdd59e052cb390f116454e9116c22021ae4a").into();
        let main_contract_id = exec::program_id();
        let match_data = MatchContractData {
            first_user: None,
            second_user: None,
            mainContractId: main_contract_id
        };

        let (match_id, _) = ProgramGenerator::create_program_with_gas_for_reply(
            submitted_code, 
            match_data.encode(), 
            GAS_FOR_CREATION_MATCH,
            0,
            0
        )
        .expect("Error during Match program initialization")
        .await
        .expect("Program wat not initialized");
                
        match_id
    }
}

impl From<ContractData> for ContractState {
    fn from(value: ContractData) -> Self {
        let ContractData {
            owner,
            match_contract,
            nft_contract,
            users,
            games,
            transaction_id
        } = value;
        
        let users = users
            .iter()
            .map(|(user_id, in_game)| (*user_id, in_game.clone()))
            .collect();
        let games = games
            .iter()
            .map(|(match_id, game_state)| (*match_id, game_state.clone()))
            .collect();
        
        Self {
            owner,
            match_contract,
            nft_contract,
            users,
            games,
            transaction_id
        }
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

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RutzoNft {
    pub media: String,
    pub nft_type: String,
    pub power: u8
}