#![no_std]
use gear_lib_old::non_fungible_token::{
    io::NFTTransfer,
    token::{TokenId, TokenMetadata},
};
use nft_io::{ NFTAction, NFTEvent };
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, collections::HashMap, msg, errors::Error, exec};

pub struct ProgramMetadata;

pub type NftContractId = ActorId;
pub type UserId = ActorId;
pub type MatchId = ActorId;
pub type TransactionId = u64;
pub type GameId = usize;
pub type InGame = bool;
pub type NFTPrice = u128;
pub type DataFromContract = Vec<(ActorId, Vec<TokenMetadata>)>;
pub type Power = u64;
pub type UserNFTS = (ActorId, Vec<TokenMetadata>);

pub const ONE_TVARA_VALUE: u128 = 1000000000000;

impl Metadata for ProgramMetadata {
    type Init = In<InitContractData>;
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Others = InOut<RutzoAction, RutzoEvent>;
    type Signal = ();
    type Reply = ();    
    type State = InOut<RutzoStateQuery, RutzoStateReply>;
}

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum MatchResult {
    PlayerOneWins,
    PlayerTwoWins,
    #[default]
    Draw
}

#[derive(Default, Encode, Decode, TypeInfo, Eq, PartialEq, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTCardType {
    #[default]
    Normal, 
    Fire, 
    Water,
    Grass, 
    Electric, 
    Ice, 
    Fight, 
    Poison,
    Ground,
    Flying,
    Psychic, 
    Bug,  
    Rock, 
    Ghost, 
    Dragon, 
    Dark, 
    Steel,
    Fairy
}

impl NFTCardType {
    pub fn batle(user1: Self, user2: Self) -> MatchResult {
        let result1 = user1.is_waek_to(user2.clone());
        let result2 = user2.is_waek_to(user1);
        if (result1 && result2) || !(result1 || result2) {
            MatchResult::Draw
        } else if result1 {
            MatchResult::PlayerTwoWins
        } else {
            MatchResult::PlayerOneWins
        }
    }
    
    pub fn is_waek_to(&self, card2: Self) -> bool {
        match *self {
            Self::Normal => {
                card2 == Self::Fight
            },
            Self::Fire => {
                card2 == Self::Water ||
                card2 == Self::Ground ||
                card2 == Self::Rock
            },
            Self::Water => {
                card2 == Self::Grass ||
                card2 == Self::Electric
            },
            Self::Grass => {
                card2 == Self::Fire ||
                card2 == Self::Ice ||
                card2 == Self::Poison ||
                card2 == Self::Flying ||
                card2 == Self::Bug
            },
            Self::Electric => {
                card2 == Self::Ground
            },
            Self::Ice => {
                card2 == Self::Fire ||
                card2 == Self::Fight ||
                card2 == Self::Rock ||
                card2 == Self::Steel
            },
            Self::Fight => {
                card2 == Self::Flying ||
                card2 == Self::Psychic ||
                card2 == Self::Fairy
            },
            Self::Poison => {
                card2 == Self::Ground ||
                card2 == Self::Psychic
            },
            Self::Ground => {
                card2 == Self::Water ||
                card2 == Self::Grass || 
                card2 == Self::Ice
            },
            Self::Flying => {
                card2 == Self::Electric ||
                card2 == Self::Ice ||
                card2 == Self::Rock
            },
            Self::Psychic => {
                card2 == Self::Bug ||
                card2 == Self::Ghost ||
                card2 == Self::Dark
            },
            Self::Bug => {
                card2 == Self::Flying ||
                card2 == Self::Rock ||
                card2 == Self::Fire
            },
            Self::Rock => {
                card2 == Self::Water ||
                card2 == Self::Grass ||
                card2 == Self::Fight ||
                card2 == Self::Ground ||
                card2 == Self::Steel
            },
            Self::Ghost => {
                card2 == Self::Ghost ||
                card2 == Self::Dark
            },
            Self::Dragon => {
                card2 == Self::Ice ||
                card2 == Self::Dragon ||
                card2 == Self::Fairy
            },
            Self::Dark => {
                card2 == Self::Fight ||
                card2 == Self::Bug ||
                card2 == Self::Fairy
            },
            Self::Steel => {
                card2 == Self::Fire ||
                card2 == Self::Fight ||
                card2 == Self::Ground
            },
            Self::Fairy => {
                card2 == Self::Poison ||
                card2 == Self::Steel
            }
        }
    }
    
    pub fn string_to_type(card_type: &str) -> Result<Self, ()> {
        match card_type {
            "Normal" => {
                Ok(Self::Normal)
            },
            "Fire" => {
                Ok(Self::Fire)
            },
            "Water" => {
                Ok(Self::Water)
            },
            "Grass" => {
                Ok(Self::Grass)
            },
            "Electric" => {
                Ok(Self::Electric)
            },
            "Ice" => {
                Ok(Self::Ice)
            },
            "Fight" => {
                Ok(Self::Fight)
            },
            "Poison" => {
                Ok(Self::Poison)
            },
            "Ground" => {
                Ok(Self::Ground)
            },
            "Flying" => {
                Ok(Self::Flying)
            },
            "Psychic" => {
                Ok(Self::Psychic)
            },
            "Bug" => {
               Ok( Self::Bug)
            },
           "Rock" => {
               Ok(Self::Rock)
           },
           "Ghost" => {
               Ok(Self::Ghost)
           },
           "Dragon" => {
              Ok(Self::Dragon)
           },
           "Dark" => {
               Ok(Self::Dark)
           },
           "Steel" => {
               Ok(Self::Steel)
           },
           "Fairy" => {
               Ok(Self::Fairy)
           },
           _ => {
               Err(())
           }
        }
    }
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTDefault {
    pub name: String,
    pub description: String,
    pub media: String,
    pub reference: String,
    pub sale_id: u64
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NFTOnSale {
    pub token_id: TokenId,
    pub value: u128
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateQuery{
    UserIsRegister(UserId),
    GameInformationById(u64),
    MatchStateById(u64),
    PlayerInformation(UserId),
    PlayerIsInMatch(ActorId),
    NFTsPurchasedByUser(UserId),
    UserCanMintDefaultsNFTs(UserId),
    NFTsOnSale,
    DefaultsNFTS,
    #[default]
    All
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateReply {
    UserIsRegister(bool),
    GameInformation(MatchInformation),
    PlayerInformation(UserDataState),
    PlayerInMatch(Option<u64>),
    MatchState(MatchState),
    PurchasedNfts(Vec<u64>),
    UserCanMintDefaultsNfts(bool),
    NFTsOnSale(Vec<NFTOnSale>),
    DefaultsNFTs(Vec<NFTDefault>),
    MatchDoesNotExists,
    UserDoesNotExists,
    All(ContractState)
}


#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoAction {
    MessageTest,
    PlayGame(TokenId),
    MintCard {
        token_id: u8
    },
    SetNFTAddress(ActorId),
    Register,
    AddNftForSale {
        token_metadata: TokenMetadata,
        value: u128
    },
    BuyNFT(TokenId),
    ApproveMinter(ActorId),
    DelegateApprovedUser(ActorId),
    SetContractToReceiveInformation(ActorId),
    SetContractToSendData(ActorId),
    RestoreInformationFromOldMainContract,
    GetAllInformation,
    DeleteContract,
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoEvent {
    MessageTestResponse,
    Minted(TokenId),
    NFTContractSaved,
    ErrorCallingNFTContract,
    UserIsNotApproved(UserId),
    UserIsNotTheOwner(UserId),
    Approved(UserId),
    UserApprovedNotExists(UserId),
    ApprovedUserDeleted(UserId),
    ErrorBuying(String),
    InsufficientFunds(NFTPrice),
    NftWithTokenIdDoesNotExists(TokenId),
    NFTWithIdNotExists(u8),
    NFTTypeDoesNotExist(String),
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
    NewPlayer(UserId),
    MatchFinished,
    MatchCreated,
    UserIsAlreadyInAGame(u64),
    PendingTransfer(TokenId),
    TransferSuccess(TokenId),
    NFTIsNotApprovedByMainContract(TokenId),
    CommunicationError(ActorId),
    MaxMintsReached(UserId),
    Profits(u128),
    ContractSet(ActorId),
    ContractIsNotTheMain,
    InformationRecovered,
    ErrorRetrievingInformation,
    WrongMessageFromNFTContract,
    NFTTypeIsIncorrect((TokenId, String)),
    ContractsData(DataFromContract),
    ContractToReceiveDataNotSet,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GameState {
    WaitigForPlayer,
    GameInProgress
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserDataState {
    pub current_game: Option<u64>,  
    pub recent_past_game: Option<u64>,
    pub past_games: Vec<u64>,
}

impl From<UserData> for UserDataState {
    fn from(value: UserData) -> Self {
        let current_game = if let Some(game_id) = value.current_game {
            Some(game_id as u64)
        } else {
            None  
        };
        let past_games = value.past_games
            .iter()
            .map(|game_id| *game_id as u64)
            .collect();
        let recent_past_game = if  value.recent_past_game.is_some() {
            let recent_past_game = value.recent_past_game.unwrap();
            Some(recent_past_game as u64)
        } else {
            None
        };
        Self {
            current_game,
            past_games,
            recent_past_game
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserDefaultMints {
    pub nfts_minted: Vec<u8>,
    pub can_mint: bool
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ContractState {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    pub games: Vec<MatchInformation>,
    pub games_waiting: Vec<u64>,
    pub games_information_by_user: Vec<(UserId, UserDataState)>,
    pub game_id: u64,
    pub tokens_metadata_default: Vec<(u8, TokenMetadata)>,
    pub nfts_for_sale: Vec<(TokenId, NFTPrice)>,
    pub default_tokens_minted_by_id: Vec<(UserId, UserDefaultMints)>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: TransactionId,
    pub pending_transfers: Vec<(UserId, (UserId, TokenId))>,
    pub sales: Vec<(ActorId, TokenId)>,
    pub contract_balance: u128,
    pub accept_data_from: Option<ActorId>,
    pub send_data_to: Option<ActorId>
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContractData {
    pub nft_contract: Option<ActorId>,
    pub tokens_metadata_default: Vec<TokenMetadata>,
    pub contract_to_receive_data: Option<ActorId>,
    pub contract_to_send_data: Option<ActorId>
}

#[derive(Default, Clone)]
pub struct UserData {
    pub current_game: Option<usize>,
    pub recent_past_game: Option<usize>,
    pub past_games: Vec<usize>,
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserGameData {
    pub user_id: ActorId,
    pub chosen_nft: TokenId,
    pub nft_type: NFTCardType,
    pub nft_power: u8,
    pub nft_data: TokenMetadata
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MatchState {
    Finished {
        winner: ActorId,
        loser: ActorId
    },
    #[default]
    InProgress,
    NotExists,
    Draw
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MatchInformation {
    pub user_1: UserGameData,
    pub user_2: Option<UserGameData>,
    pub match_state: MatchState,
}

#[derive(Default)]
pub struct Contract {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    pub games: Vec<MatchInformation>,
    pub games_waiting: Vec<GameId>,
    pub games_information_by_user: HashMap<UserId, UserData>,
    pub game_id: GameId,
    pub tokens_metadata_default: HashMap<u8, TokenMetadata>,
    pub nfts_for_sale: HashMap<TokenId, NFTPrice>,
    pub default_tokens_minted_by_id: HashMap<UserId, UserDefaultMints>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: TransactionId,
    pub pending_transfers: HashMap<UserId, (UserId, TokenId)>,
    pub sales: HashMap<ActorId, TokenId>,
    pub contract_balance: u128,
    pub accept_data_from: Option<ActorId>,
    pub send_data_to: Option<ActorId>
}

impl Contract {
    pub async fn burn_all_nfts(nft_contract: ActorId) -> Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::BurnAllNFTS,
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Mint' to nft contract")
        .await
    }
    
    pub async fn data_from_old_contract(contract: ActorId) -> Result<RutzoEvent, Error> {
        msg::send_for_reply_as::<RutzoAction, RutzoEvent>(
            contract, 
            RutzoAction::GetAllInformation, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Mint' to nft contract")
        .await
    }
    
    pub async fn mint_tokens_to_user(nft_contract: ActorId, to: UserId, nfts: Vec<TokenMetadata>) -> Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::MintNFTsTo { 
                to, 
                nfts 
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Mint' to nft contract")
        .await
    }
    
    pub async fn nft_data_from_users(nft_contract: ActorId, users: Vec<UserId>) ->Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::NFTDataFromUsers(users), 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Mint' to nft contract")
        .await
    }
    
    pub async fn mint_new_nft(nft_contract: ActorId, token_metadata: TokenMetadata, transaction_id: u64) -> Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::Mint { 
                transaction_id, 
                token_metadata
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Mint' to nft contract")
        .await
    }
    
    pub async fn transfer_match_nft(nft_contract: ActorId, to: ActorId, token_id: TokenId, transaction_id: u64) -> Result<NFTEvent, ()>{
        match msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::TranserNFTToUser { 
                transaction_id, 
                to, 
                token_id
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Transfer' to nft contract")
        .await {
            Ok(nft_event) => {
                let NFTEvent::Transfer(_) = nft_event else { 
                    panic!("Unexpected answer from nft contract"); 
                };
                Ok(nft_event)
            },
            Err(_) => Err(())
        }
    }
    
    pub async fn transfer_nft(nft_contract: ActorId, to: ActorId, token_id: TokenId, transaction_id: u64) -> Result<NFTEvent, ()>{
        match msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::Transfer { 
                transaction_id, 
                to, 
                token_id
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::Transfer' to nft contract")
        .await {
            Ok(nft_event) => {
                let NFTEvent::Transfer(_) = nft_event else { 
                    panic!("Unexpected answer from nft contract"); 
                };
                Ok(nft_event)
            },
            Err(_) => Err(())
        }
    }
    
    pub async fn main_contract_is_approved(nft_contract: ActorId, main_contract: ActorId, token_id: TokenId) -> Result<bool, ()> {
        match msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::IsApproved { 
                to: main_contract,
                token_id 
            }, 
            0, 
            0
        )
        .expect("Error in sending a message 'NFTAction::MintFromMainContract' to a match contract")
        .await {
            Ok(nft_event) => {
                let NFTEvent::IsApproved { approved, .. } = nft_event else { 
                    panic!("Unexpected answer from nft contract"); 
                };
                Ok(approved)
            },
            Err(_) => Err(())
        }
    }
    
    pub async fn nft_data_by_tokenid(nft_contract: ActorId, token_id: TokenId) -> Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::NFTData(token_id),
             0, 
             0
        )
        .expect("Error in sending a message 'NFTAction::Transfer' to nft contract")
        .await
    }
}

impl Contract {
    pub async fn all_data(&self, caller: ActorId) -> RutzoEvent {
        if caller != self.owner && (self.send_data_to.is_none() || caller != self.send_data_to.unwrap())  {
            return RutzoEvent::UserIsNotApproved(caller);
        }
        
        let mut users: Vec<ActorId> = self.games_information_by_user
            .keys()
            .map(|user_id| user_id.clone())
            .collect();
        
        users.push(exec::program_id());
        
        let answer = Contract::nft_data_from_users(
            self.nft_contract.unwrap(),
            users
        )
        .await
        .expect("Unable to decode NFTEvent");
        
        let NFTEvent::AllNFTInformation(users_nfts) = answer else {
            panic!("Answer is not the correct!"); 
        };
        
        RutzoEvent::ContractsData(users_nfts)
    }
    
    pub async fn restore_data(&mut self, caller: ActorId) -> RutzoEvent {
        if caller != self.owner {
            return RutzoEvent::UserIsNotTheOwner(caller);
        }
        
        if self.accept_data_from.is_none() {
            return RutzoEvent::ContractToReceiveDataNotSet;
        }
        
        let answer = Contract::data_from_old_contract(
            self.accept_data_from.unwrap()
        )
        .await
        .expect("Unable to decode RutzoEvent");
        
        let RutzoEvent::ContractsData(data_to_restore) = answer else {
            panic!("Answer is not the correct!");  
        };
        
        let mut error_retrieving_data = false;
        let mut users_restored_data = Vec::new();
        for (user, tokens_metadata) in data_to_restore.into_iter() {
            let answer = Contract::mint_tokens_to_user(
                self.nft_contract.unwrap(), 
                user, 
                tokens_metadata
            )
            .await
            .expect("Unable to decode RutzoEvent");
            
            users_restored_data.push(user);
            if user != exec::program_id() {
                self.register_user(user);
            }
            
            if answer != NFTEvent::NFTsMinted {
                error_retrieving_data = true;
                break;
            }
        }
        
        if error_retrieving_data {
            Contract::burn_all_nfts(
                self.nft_contract.unwrap(),
            )
            .await
            .expect("Error decodind NFTEvent");
            
            self.games_information_by_user.clear();
            self.default_tokens_minted_by_id.clear();
            
            return RutzoEvent::ErrorRetrievingInformation;
        }
        
        RutzoEvent::InformationRecovered
    }
    
    pub async fn buy_nft(&mut self, user_id: UserId, token_id: TokenId, value: NFTPrice) -> (RutzoEvent, u128) {
        if !self.is_register(&user_id) {
            return (RutzoEvent::AccountNotExists(user_id), value);
        }
        
        let token_on_sale_value = match self.nfts_for_sale.get(&token_id) {
            Some(value) => (*value) * ONE_TVARA_VALUE,
            None => return (RutzoEvent::NftWithTokenIdDoesNotExists(token_id), value)
        };
        
        if value < token_on_sale_value {
            return (RutzoEvent::InsufficientFunds(value), value);
        }
        
        let value_to_return = value - token_on_sale_value;
        self.contract_balance = self.contract_balance.saturating_add(token_on_sale_value);

        
        Contract::transfer_nft(
            self.nft_contract.unwrap(), 
            user_id, 
            token_id, 
            self.transaction_id
        )
        .await
        .expect("Unable to decode NFTEvent");
        
        self.transaction_id = self.transaction_id.saturating_add(1);
        
        self.nfts_for_sale.remove(&token_id);
        
        if self.contract_balance > 15* ONE_TVARA_VALUE {
            let profit = self.contract_balance;
            self.contract_balance = 0;
            msg::send(self.owner, RutzoEvent::Profits(profit), profit)
                .expect("Error sending profits to owner!");
        }
        
        (RutzoEvent::NFTContractSaved, value_to_return)
    }
    
    pub async fn mint_nft_to_sale(&mut self, user_id: ActorId, token_metadata: TokenMetadata, value: NFTPrice) -> RutzoEvent {
        let user_approved = self.approved_minters 
            .iter()
            .find(|&approved_user_id| *approved_user_id == user_id);
        
        if user_approved.is_none() && self.owner != user_id {
            return RutzoEvent::UserIsNotApproved(user_id);
        }
        
        let answer = Contract::mint_new_nft(
            self.nft_contract.unwrap(), 
            token_metadata, 
            self.transaction_id
        )
        .await
        .expect("Unable to decode NFTEvent");
        
        let NFTEvent::Transfer(NFTTransfer { token_id, .. }) = answer else {
            panic!("Unable to decode NFTEvent");  
        };
        
        self.nfts_for_sale.insert(token_id, value);
        self.transaction_id = self.transaction_id.saturating_add(1);
        
        RutzoEvent::Minted(token_id)
    }
    
    pub async fn mint_card(&mut self, token_id: u8) -> RutzoEvent {
        let user_id = msg::source();
        
        if !self.is_register(&user_id) {
            return RutzoEvent::AccountNotExists(user_id);
        } 
        
        let nfts_minted_data = self.default_tokens_minted_by_id.get_mut(&user_id).unwrap();
        
        if nfts_minted_data.can_mint && nfts_minted_data.nfts_minted.len() == 3 {
            return RutzoEvent::MaxMintsReached(user_id);    
        }
    
        let token_metadata = self.tokens_metadata_default
            .get(&token_id);
           
        let token_metadata = match token_metadata {
            Some(token_data) => token_data.clone(),
            None => return RutzoEvent::NFTWithIdNotExists(token_id)
        };
        
        let answer = Contract::mint_new_nft(
            self.nft_contract.unwrap(), 
            token_metadata, 
            self.transaction_id
        )
        .await
        .expect("Unable to decode NFTEvent");
        
        let NFTEvent::Transfer(transfer_data) = answer else {
            return RutzoEvent::ErrorCallingNFTContract; 
        };
        
        self.transaction_id = self.transaction_id.saturating_add(1);
        
        Contract::transfer_nft(
            self.nft_contract.unwrap(),
            user_id, 
            transfer_data.token_id,
            self.transaction_id
        )
        .await
        .expect("Unable to decode NFTEvent");
        
        self.default_tokens_minted_by_id
            .entry(user_id)
            .and_modify(|minted_data| {
                minted_data.nfts_minted.push(token_id);
                if minted_data.nfts_minted.len() == 3 {
                    minted_data.can_mint = false;
                }
            })
            .or_default();
        
        self.transaction_id = self.transaction_id.saturating_add(1);
        
        RutzoEvent::Minted(TokenId::default())
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
      
        self.games_information_by_user.insert(user_id, UserData {
            current_game: None,
            recent_past_game: None,
            past_games: vec![]
        });       
        self.default_tokens_minted_by_id.insert(user_id, UserDefaultMints {
            nfts_minted: Vec::new(),
            can_mint: true
        });
        RutzoEvent::RegisterSucces
    }
    
    pub fn is_register(&self, user_id: &UserId) ->  bool {
        self.games_information_by_user.contains_key(user_id)
    }
    
    //pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId, power: u8) -> RutzoEvent {
    pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId) -> RutzoEvent {
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
            
        if let Some(&(to, token_id)) = self.pending_transfers.get(&user_id) {
            if Contract::transfer_match_nft(
                self.nft_contract.unwrap(),
                to, 
                token_id,
                self.transaction_id
            ).await.is_err() {
                return RutzoEvent::PendingTransfer(token_id);
            }
            
            self.transaction_id = self.transaction_id.saturating_add(1);
            self.pending_transfers.remove(&user_id);
            
            return RutzoEvent::TransferSuccess(token_id);
        }
            
        let answer = Contract::nft_data_by_tokenid(
            self.nft_contract.unwrap(), 
            token_id
        ).await;
                                                            
        if answer.is_err() {
            return RutzoEvent::CommunicationError(self.nft_contract.unwrap());
        }
                                
        if let  NFTEvent::TokenIdNotExists(token_id) = answer.clone().unwrap() {
            return RutzoEvent::NftWithTokenIdDoesNotExists(token_id);
        }
                                
        let answer = if answer.clone().unwrap() == NFTEvent::ActionOnlyForMainContract {
            return RutzoEvent::ContractIsNotTheMain;
        } else {
            answer.unwrap()  
        };
                                     
        let nft_data = match answer {
            NFTEvent::NFTData(nft_token_metadata) => {
                 if let Some(data) = nft_token_metadata {
                    data
                } else {
                     return RutzoEvent::NftWithTokenIdDoesNotExists(token_id); 
                }
            },
            _ => {
                return RutzoEvent::WrongMessageFromNFTContract;
            }
        };
                                
        let nft_type = if let Ok(card_type) = NFTCardType::string_to_type(&nft_data.description) {
            card_type
        } else {
            return RutzoEvent::NFTTypeIsIncorrect((token_id, nft_data.description));
        };
        
        let nft_power: u8 = nft_data.reference
            .parse()
            .expect("error parsing power");
        
        let user_data = self.games_information_by_user
            .get_mut(&user_id)
            .unwrap();
        
        if user_data.current_game.is_some() {
            return RutzoEvent::UserIsAlreadyInAGame(
                user_data.current_game.unwrap() as u64
            );
        }
        
        let (game_data, game_id) = match self.games_waiting.pop() {
            None => {
                self.games.push(MatchInformation {
                    user_1: UserGameData {
                        user_id,
                        chosen_nft: token_id,
                        nft_type,
                        nft_power,
                        nft_data
                    },
                    user_2: None,
                    match_state: MatchState::default()
                });
                self.games_waiting.push(self.game_id);
                self.set_player_in_current_game(user_id, self.game_id);
                self.game_id = self.game_id.saturating_add(1);
                
                return RutzoEvent::MatchCreated;
            },
            Some(game_id) => {                
                (&mut self.games[game_id], game_id)
            }
        };
        
        game_data.user_2 = Some(UserGameData {
            user_id,
            chosen_nft: token_id,
            nft_type: nft_type.clone(),
            nft_power,
            nft_data
        });
        
        let user1_power = game_data.user_1.nft_power;
        let user1_card_type = game_data.user_1.nft_type.clone();
        let user1_id = game_data.user_1.user_id;
        
        let (winner, loser, token_id, is_draw) = match NFTCardType::batle(user1_card_type, nft_type) {
            MatchResult::Draw => {
                if user1_power == nft_power {
                    (ActorId::default(), ActorId::default(), TokenId::default(), true)
                } else if  user1_power > nft_power {
                    let winner = user1_id;
                    let loser = user_id;
                    let token_id = token_id;
                    (winner, loser, token_id, false)
                } else {
                    let winner = user_id;
                    let loser = user1_id;
                    let token_id = game_data.user_1.chosen_nft;
                    (winner, loser, token_id, false)
                }
            },
            MatchResult::PlayerOneWins => (user1_id, user_id, token_id, false),
            MatchResult::PlayerTwoWins => (user_id, user1_id, game_data.user_1.chosen_nft, false)
        };
                
        if !is_draw {
            game_data.match_state = MatchState::Finished { 
                winner, 
                loser
            };  
        } else {
            game_data.match_state = MatchState::Draw;
        }
    
        self.player_finish_game(winner, game_id);
        self.player_finish_game(loser, game_id);
        
        if !is_draw {
            if Contract::transfer_nft(
                self.nft_contract.unwrap(),
                winner, 
                token_id,
                self.transaction_id
            ).await.is_err() {
                self.pending_transfers.insert(loser, (winner, token_id));
                return RutzoEvent::PendingTransfer(token_id);
            }
            
            self.transaction_id = self.transaction_id.saturating_add(1);
        }
        
        RutzoEvent::MatchFinished
    }
    
    pub fn set_player_in_current_game(&mut self, user_id: ActorId, game_id: usize) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.current_game = Some(game_id);
                user_game_data.recent_past_game = None;
            });
    }
    
    pub fn player_finish_game(&mut self, user_id: ActorId, game_id: usize) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.recent_past_game = Some(game_id);
                user_game_data.current_game = None;
                user_game_data.past_games.push(game_id);
            });
    }
}

impl From<Contract> for ContractState {
    fn from(value: Contract) -> Self {
        let Contract {
            owner,
            nft_contract,
            games_information_by_user,
            games_waiting,
            games,
            game_id,
            pending_transfers,
            default_tokens_minted_by_id,
            nfts_for_sale,
            tokens_metadata_default,
            approved_minters,
            transaction_id,
            sales,
            contract_balance,
            accept_data_from,
            send_data_to
        } = value;

        let games_waiting = games_waiting
            .iter()
            .map(|match_id| *match_id as u64)
            .collect();
        let tokens_metadata_default = tokens_metadata_default
            .iter()
            .map(|(token_id, token_metadata)| (*token_id, token_metadata.clone()))
            .collect();
        let default_tokens_minted_by_id = default_tokens_minted_by_id
            .iter()
            .map(|(user_id, minted)| (*user_id, minted.clone()))
            .collect();
        let games_information_by_user = games_information_by_user
            .iter()
            .map(|(user_id, user_data)| (*user_id, user_data.clone().into()))
            .collect();
        let pending_transfers = pending_transfers
            .iter()
            .map(|(user_id, token_id)| (*user_id, *token_id))
            .collect();
        let nfts_for_sale = nfts_for_sale 
            .iter()
            .map(|(token_id, price)| (*token_id, *price) )
            .collect();
        let game_id = game_id as u64;
        let sales = sales
            .iter()
            .map(|(user_id, nft_id)| (*user_id, *nft_id))
            .collect();
        
        Self {
            owner,
            nft_contract,
            games,
            games_waiting,
            games_information_by_user,
            game_id,
            tokens_metadata_default,
            nfts_for_sale,
            default_tokens_minted_by_id,
            approved_minters,
            transaction_id,
            pending_transfers,
            sales,
            contract_balance,
            accept_data_from,
            send_data_to
        }
    }
}




