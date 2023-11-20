#![no_std]
use gear_lib_old::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    delegated::DelegatedApproveMessage,
    royalties::*,
    token::{TokenId, TokenMetadata},
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, collections::BTreeMap, msg, errors::Error, exec};
use primitive_types::H256;

pub struct ProgramMetadata;

pub type NftContractId = ActorId;
pub type UserId = ActorId;
pub type MatchId = ActorId;
pub type TransactionId = u64;
pub type GameId = usize;
pub type InGame = bool;
pub type NFTPrice = u128;
pub type DataFromContract = Vec<(ActorId, Vec<TokenMetadata>)>;

impl Metadata for ProgramMetadata {
    type Init = In<InitContractData>;
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Others = InOut<RutzoAction, RutzoEvent>;
    type Signal = ();
    type Reply = ();    
    type State = InOut<RutzoStateQuery, RutzoStateReply>;
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateQuery{
    GameInformationById(u64),
    PlayerInformation(UserId),
    #[default]
    All
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoStateReply {
    GameInformation(MatchInformation),
    PlayerInformation(UserDataState),
    All(ContractState),
    MatchDoesNotExists,
    UserDoesNotExists
}


#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RutzoAction {
    MessageTest,
    PlayGame {
        token_id: TokenId,
        power: String
    },
    MintCard {
        token_id: u8
    },
    SetNFTAddress(ActorId),
    Register,
    AddNftForSale {
        token_metadata: TokenMetadata
    },
    BuyNFT(TokenId),
    ApproveMinter(ActorId),
    DelegateApprovedUser(ActorId),
    SetContractToReceiveInformation(ActorId),
    SetInformationFromOldContract(DataFromContract)
}

#[derive(Encode, Decode, TypeInfo)]
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
    MaxMintsReached(UserId),
    Profits(u128),
    ContractSet(ActorId),
    InformationRecovered,
    ErrorRetrievingInformation
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
    current_game: Option<u64>,  
    recent_past_game: Option<u64>,
    past_games: Vec<u64>,
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
    nfts_minted: Vec<u8>,
    can_mint: bool
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
    pub accept_data_from: ActorId
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContractData {
    pub nft_contract: Option<ActorId>,
    pub tokens_metadata_default: Vec<TokenMetadata>,
}

#[derive(Default, Clone)]
pub struct UserData {
    current_game: Option<usize>,
    recent_past_game: Option<usize>,
    past_games: Vec<usize>,
}

#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserGameData {
    pub user_id: ActorId,
    pub chosen_nft: TokenId,
    pub power: u8,
    
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
    NotExists
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
    pub games_information_by_user: BTreeMap<UserId, UserData>,
    pub game_id: GameId,
    pub tokens_metadata_default: BTreeMap<u8, TokenMetadata>,
    pub nfts_for_sale: BTreeMap<TokenId, NFTPrice>,
    pub default_tokens_minted_by_id: BTreeMap<UserId, UserDefaultMints>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: TransactionId,
    pub pending_transfers: BTreeMap<UserId, (UserId, TokenId)>,
    pub sales: BTreeMap<ActorId, TokenId>,
    pub contract_balance: u128,
    pub accept_data_from: ActorId
}

impl Contract {
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
}

impl Contract {
    pub async fn restore_data(&mut self, contract_data: DataFromContract) -> RutzoEvent {
        let source = msg::source();
        if source != self.accept_data_from {
            return RutzoEvent::UserIsNotApproved(source);
        }
            
        for (user_id, tokens_metadata) in contract_data.into_iter() {
            for token_metadata in tokens_metadata.into_iter() {
                let answer_mint = Contract::mint_new_nft(self.nft_contract.unwrap(), token_metadata, self.transaction_id).await;
                if answer_mint.is_err() {
                    return RutzoEvent::ErrorRetrievingInformation;
                }
                self.transaction_id = self.transaction_id.saturating_add(1);
                let NFTEvent::Transfer(NFTTransfer { token_id, .. }) = answer_mint.unwrap() else {
                    panic!("Answer is not the correct!");
                };
                if Contract::transfer_nft(self.nft_contract.unwrap(), user_id, token_id, self.transaction_id)
                .await.is_err() {
                    return RutzoEvent::ErrorRetrievingInformation;
                }
                self.transaction_id = self.transaction_id.saturating_add(1);
            }
            self.register_user(user_id);
        }
        
        RutzoEvent::InformationRecovered
    }
    
    pub async fn buy_nft(&mut self, user_id: UserId, token_id: TokenId, value: NFTPrice) -> (RutzoEvent, u128) {
        if !self.is_register(&user_id) {
            return (RutzoEvent::AccountNotExists(user_id), value);
        }
        
        let token_on_sale_value = match self.nfts_for_sale.get(&token_id) {
            Some(value) =>* value,
            None => return (RutzoEvent::NftWithTokenIdDoesNotExists(token_id), value)
        };
        
        if value < token_on_sale_value {
            return (RutzoEvent::InsufficientFunds(value), value);
        }
        
        let value_to_return = value - token_on_sale_value;
        self.contract_balance = self.contract_balance.saturating_add(value - value_to_return);

        if self.contract_balance > 10 {
            let profit = self.contract_balance;
            msg::send(self.owner, RutzoEvent::Profits(profit), profit)
                .expect("Error sending profits to owner!");
            self.contract_balance = 0;
        }
        
        Contract::transfer_nft(
            self.nft_contract.unwrap(), 
            user_id, 
            token_id, 
            self.transaction_id
        )
        .await
        .expect("Unable to decode NFTEvent");
        
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
            nfts_minted_data.can_mint = false;
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
            .and_modify(|minted_data| minted_data.nfts_minted.push(token_id))
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
      
        self.games_information_by_user.insert(user_id, Default::default());       
        self.default_tokens_minted_by_id.insert(user_id, Default::default());
        RutzoEvent::RegisterSucces
    }
    
    pub fn is_register(&self, user_id: &UserId) ->  bool {
        self.games_information_by_user.contains_key(user_id)
    }
    
    pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId, power: u8) -> RutzoEvent {
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
            
        if let Ok(approved) = Contract::main_contract_is_approved(
            self.nft_contract.unwrap().clone(), 
            exec::program_id(),
            token_id
        ).await {
            if !approved {
                return RutzoEvent::NFTIsNotApprovedByMainContract(token_id);
            }
        } else {
            return RutzoEvent::NFTIsNotApprovedByMainContract(token_id);
        }
        
        if let Some(&(to, token_id)) = self.pending_transfers.get(&user_id) {
            if Contract::transfer_nft(
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
                        power
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
            power,
        });
        
        let user1_power = game_data.user_1.power;
        let user1_id = game_data.user_1.user_id;
        
        let (winner, loser, token_id) = if   user1_power > power {
            let winner = user1_id;
            let loser = user_id;
            let token_id = token_id;
            (winner, loser, token_id)
        } else {
            let winner = user_id;
            let loser = user1_id;
            let token_id = game_data.user_1.chosen_nft;
            (winner, loser, token_id)
        };
                
        game_data.match_state = MatchState::Finished { 
            winner, 
            loser
        };
        
        self.player_finish_game(winner, game_id);
        self.player_finish_game(loser, game_id);
        
        /*
        self.add_past_game_to_player(user_id, game_id);
        self.add_past_game_to_player(user1_id, game_id);
        self.delete_player_current_game(winner);
        self.delete_player_current_game(loser);
                */
                
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
        
        RutzoEvent::MatchFinished
    }
    
    pub fn set_player_in_current_game(&mut self, user_id: ActorId, game_id: usize) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.current_game = Some(game_id);
                user_game_data.recent_past_game = None;
            })
            .or_default();
    }
    
    pub fn player_finish_game(&mut self, user_id: ActorId, game_id: usize) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.past_games.push(game_id);
                user_game_data.recent_past_game = Some(game_id);
                user_game_data.current_game = None;
            })
            .or_default();
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
            accept_data_from
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
            accept_data_from
        }
    }
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
    NFTData(TokenId),
    NFTDataFromUsers(Vec<ActorId>),
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
    AllNFTInformation(Vec<(ActorId, Vec<TokenMetadata>)>),
    ActionOnlyForMainContract,
    MainContractIsNotApproved
}

