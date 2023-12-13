#![no_std]
use gear_lib_old::non_fungible_token::{
    io::NFTTransfer,
    token::{TokenId, TokenMetadata},
};
use nft_io::{ NFTAction, NFTEvent };
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, collections::HashMap, msg, errors::Error, exec};

pub mod contract_types;
use contract_types::*;

pub mod contract_utils;
use contract_utils::*;

pub mod main_actions_events;
use main_actions_events::*;

pub mod main_state_actions_events;
use main_state_actions_events::*;

pub mod nft_utils;
use nft_utils::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitContractData>;
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Others = InOut<RutzoAction, RutzoEvent>;
    type Signal = ();
    type Reply = ();    
    type State = InOut<RutzoStateQuery, RutzoStateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ContractState {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    //pub games: Vec<MatchInformation>,
    pub games: Vec<GameData>,
    pub games_waiting: Vec<u64>,
    pub games_information_by_user: Vec<(UserId, UserDataState)>,
    pub game_id: u64,
    pub tokens_metadata_default: Vec<(u8, TokenMetadata)>,
    pub nfts_for_sale: Vec<(TokenId, NFTPrice)>,
    pub default_tokens_minted_by_id: Vec<(UserId, UserDefaultMints)>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: TransactionId,
    pub pending_transfers: Vec<(UserId, (UserId, u64))>,
    pub sales: Vec<(ActorId, TokenId)>,
    pub contract_balance: u128,
    pub accept_data_from: Option<ActorId>,
    pub send_data_to: Option<ActorId>
}

#[derive(Default)]
pub struct Contract {
    pub owner: ActorId,
    pub nft_contract: Option<NftContractId>,
    pub games: Vec<MatchInformation>,
    pub games2: Vec<GameData>,
    pub games_waiting: Vec<GameId>,
    pub games_information_by_user: HashMap<UserId, UserData>,
    pub game_id: GameId,
    pub tokens_metadata_default: HashMap<u8, TokenMetadata>,
    pub nfts_for_sale: HashMap<TokenId, NFTPrice>,
    pub default_tokens_minted_by_id: HashMap<UserId, UserDefaultMints>,
    pub approved_minters: Vec<UserId>,
    pub transaction_id: TransactionId,
    pub pending_transfers: HashMap<UserId, (UserId, GameId)>,
    pub sales: HashMap<ActorId, TokenId>,
    pub contract_balance: u128,
    pub accept_data_from: Option<ActorId>,
    pub send_data_to: Option<ActorId>
}

impl Contract {
    pub fn player_contains_selected_card(nfts_chosen: &Vec<CardData>, nft_id: TokenId) -> bool {
        nfts_chosen
            .iter()
            .find(|card_data| card_data.nft_token_id == nft_id)
            .is_some()
    }
    
    pub fn start_round(card1_type: NFTCardType, card1_power: Power, card2_type: NFTCardType, card2_power: Power) -> MatchResult {
        let round_data = NFTCardType::batle(card1_type, card2_type);
        
        if round_data != MatchResult::Draw {
            return round_data;
        }
        
        if card1_power == card2_power {
            MatchResult::Draw
        } else if  card1_power > card2_power {
            MatchResult::PlayerOneWins
        } else {
            MatchResult::PlayerTwoWins
        }
    }
    
    pub fn game_with_bot(_game_data: &mut GameData, _card_id: TokenId) -> RutzoEvent {
        /*
        
        let Some(card_data) = game_data
            .user_1
            .nfts_chosen
            .iter()
            .find(|card_data| card_data.nft_token_id == card_id) else {
            return RutzoEvent::NFTSelectedIsNotInCardsSelected(card_id);
        };
        
        let CardData {
            nft_type,   
            nft_power,
            ..
        } = card_data;
        */
        
        
        RutzoEvent::ContractToReceiveDataNotSet
        
    }
    
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
    
    pub async fn nfts_data_by_token_id(nft_contract: ActorId, tokens_id: Vec<TokenId>, user: UserId) -> Result<NFTEvent, Error> {
        msg::send_for_reply_as::<NFTAction, NFTEvent>(
            nft_contract, 
            NFTAction::NFTDataFromTokensIdbyUser {
                tokens_id,
                user
            },
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
        
        if !nfts_minted_data.can_mint {
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
                if minted_data.nfts_minted.len() == 5 {
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
            past_games: vec![],
            is_logged_id: true
        });       
        self.default_tokens_minted_by_id.insert(user_id, UserDefaultMints {
            nfts_minted: Vec::new(),
            can_mint: true
        });
        RutzoEvent::RegisterSucces
    }
    
    pub fn login_user(&mut self, user_id: UserId) -> RutzoEvent {
        // Check if the user is register
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        let user_data = self.games_information_by_user
            .get_mut(&user_id)
            .unwrap();
        
        if user_data.is_logged_id {
            return RutzoEvent::AccountAlreadyLoggedIn(user_id);
        }
        
        user_data.is_logged_id = true;
        
        RutzoEvent::LoginSucces
    }  
    
    pub fn logout_user(&mut self, user_id: UserId) -> RutzoEvent {
        // Check if the user is register
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        let user_data = self.games_information_by_user
            .get_mut(&user_id)
            .unwrap();
        
        if !user_data.is_logged_id {
            return RutzoEvent::UserIsNotLoggedIn(user_id);
        }
        
        user_data.is_logged_id = false;
        
        RutzoEvent::LogoutSuccess
    }  
    
    pub fn is_register(&self, user_id: &UserId) ->  bool {
        self.games_information_by_user.contains_key(user_id)
    }
    
    pub async fn send_nft_to_winner(&mut self, user_id: UserId, nft_id: TokenId) -> RutzoEvent {
        // Check if the user is register
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        let Some((user_winner, game_id)) = self.pending_transfers.get(&user_id) else {
          return RutzoEvent::UserDoesNotHasPendingTransfer;  
        };
        
        let Some(game_data) = self.games2.get(*game_id) else {
            return RutzoEvent::GameIdDoesNotExists(*game_id as u64);
        };
        
        let user_chosen_cards =  if game_data.user_1.user_id == user_id {
            &game_data.user_1.nfts_chosen
        } else {
            &game_data.user_2.as_ref().unwrap().nfts_chosen
        };
        
        let Some(CardData { nft_token_id, .. }) = user_chosen_cards.iter().find(|&card_data| card_data.nft_token_id == nft_id) else {
            return RutzoEvent::NftIdIsNotInGamId {
                nft_id,
                game_id: *game_id as u64
            };
        };
        
        if Contract::transfer_match_nft(
            self.nft_contract.unwrap(),
            *user_winner, 
            *nft_token_id,
            self.transaction_id
        ).await.is_err() {
            return RutzoEvent::PendingTransfer;
        }
        
        self.transaction_id = self.transaction_id.saturating_add(1);
        self.pending_transfers.remove(&user_id);
        
        RutzoEvent::TransferSuccess(nft_id)
    }
    
    // Function to create a match, and checking if the user is already in a match
    pub async fn join_game(&mut self, user_id: UserId, tokens_id: Vec<TokenId>, play_with_bot: bool) -> RutzoEvent {
        // Check if the user is register
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        // Check if the user has a NFT pending transfer
        if self.pending_transfers.get(&user_id).is_some() {
            return RutzoEvent::PendingTransfer;
        }
        
        // Check if the user is already in a game 
        if let Some(user_current_game) = self.user_actual_game_id(user_id) {
            return RutzoEvent::UserIsAlreadyInAGame(
                user_current_game as u64
            );
        } 
        
        // Check if total NFT send to the action is less or greater than 3
        if tokens_id.len() > 3 || tokens_id.len() < 3 {
            return RutzoEvent::TotalTokensIdIsIncorrect(tokens_id);
        }
        
        // Get the answer of NFT data, if error, end the action        
        let Ok(answer) = Contract::nfts_data_by_token_id(
            self.nft_contract.unwrap(), 
            tokens_id,
            user_id
        ).await else {
            return RutzoEvent::CommunicationError(self.nft_contract.unwrap());
        };
        
        // If answer is that the contract is not the main, end the funcion
        if answer == NFTEvent::ActionOnlyForMainContract {
            return RutzoEvent::ContractIsNotTheMain;
        }
        
        // Get data from the message, if the message is not the correct, end the function
        // Is important because at the same time, it chekcs if he has the three nfts
        let NFTEvent::NFTsData(nfts_tokens_metadata) = answer else {
            return RutzoEvent::WrongMessageFromNFTContract;
        };
        
        // Creating new vector for the cards of the player
        let mut nfts_chosen = Vec::new();
        
        // Format the three cards for the game
        for (nft_token_id, token_metadata) in nfts_tokens_metadata.into_iter() {  
            // If one card is None, end the method          
            let Some(nft_data) = token_metadata else {
                return RutzoEvent::NftWithTokenIdDoesNotExists(nft_token_id);
            };
                
            // Getting the type of type of the card
            let Ok(nft_type) = NFTCardType::string_to_type(&nft_data.description) else {
                return RutzoEvent::NFTTypeIsIncorrect((nft_token_id, nft_data.description));
            };
            
            // Getting the power of the card
            let nft_power: Power = nft_data.reference
                .parse()
                .expect("error parsing power");
            
            // Format the information of the card fot the game
            let card_data = CardData {
                nft_token_id,
                nft_type,
                nft_power,
                nft_data
            };
                
            // Save the formated information of the card
            nfts_chosen.push(card_data);
        }
        
        // Data is ready to storage in a match
        let user_game_data = UserGameData2 {
            user_id,
            nfts_chosen
        };
        
        // If player plays with bot, create match without adding it in pendings
        // matchs and finish the function
        if play_with_bot {
            self.set_player_in_current_game(user_id, self.game_id);
            self.create_game_and_set_player_data(user_game_data, play_with_bot);
            return RutzoEvent::MatchJoined;
        }
        
        // Check if a match exists, if not, create one and join user to the match.
        let game_id = if let Some(game_id) = self.games_waiting.pop() {
            self.games2[game_id].user_2 = Some(user_game_data);
            self.games2[game_id].match_state = MatchState::InProgress;
            game_id
        } else {
            let actual_game_id = self.game_id;
            self.games_waiting.push(actual_game_id);
            self.create_game_and_set_player_data(user_game_data, play_with_bot);
            actual_game_id
        };
        
        self.set_player_in_current_game(user_id, game_id);
        
        RutzoEvent::MatchJoined
    }
    
    // Method create a new game and join the current user to that game
    pub fn create_game_and_set_player_data(&mut self, user_game_data: UserGameData2, playing_with_bot: bool) {
        let mut new_game = GameData {
            user_1: user_game_data,
            playing_with_bot, 
            ..Default::default()
        };
        
        if !playing_with_bot {
            new_game.match_state = MatchState::LookingForEnemy;          
        }
        
        self.games2.push(new_game);
        self.game_id = self.game_id.saturating_add(1);
    }
    
    // Function to call when user is already in a match and the user chose a card
    pub async fn throw_card(&mut self, user_id: UserId, card_id: TokenId) -> RutzoEvent {
        // Check if the user is register
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
        
        // Get user actual game id, if it does not exists, finish the method
        let Some(user_current_game) = self.user_actual_game_id(user_id) else {
            return RutzoEvent::UserIsNotInAGame(user_id);
        };
        
        // Get game data, if it does not exist or it throw an error, finish the method
        let Some(game_data) = self.games2.get_mut(user_current_game) else {
            return RutzoEvent::GameIdDoesNotExists(user_current_game as u64);            
        };
        
        if game_data.playing_with_bot {
            if !Contract::player_contains_selected_card(&game_data.user_1.nfts_chosen, card_id) {
               return RutzoEvent::NFTSelectedIsNotInCardsSelected(card_id);
            }
            return Contract::game_with_bot(game_data, card_id); 
            /*
            match Contract::game_with_bot(game_data, card_id) {
                MatchResult::PlayerOneWins => {
                    return RutzoEvent::PlayerOneWin;
                },
                MatchResult::PlayerTwoWins => {
                    return RutzoEvent::PlayerTwoWin;
                },
                _ => {
                    return RutzoEvent::RoundDraw;
                }
            }
            */
        }
        
        // If user2 is none, it means that the game is serching an enemy
        if game_data.user_2.is_none() {
            return RutzoEvent::GameIsWaitingPlayer(user_current_game as u64);
        }
        
        // check if the actual caller is user1
        if game_data.user_1.user_id == user_id {
            // If user already throw a card, finish the method
            if game_data.round_data.user1_card.is_some() {
                return RutzoEvent::UserAlreadyChoseACard;
            }
            // Check if the user nft selected exists in his cards selected
            if !Contract::player_contains_selected_card(&game_data.user_1.nfts_chosen, card_id) {
               return RutzoEvent::NFTSelectedIsNotInCardsSelected(card_id);
            }
            game_data.round_data.user1_card = Some(card_id);
        } else {
            // If user already throw a card, finish the method
            if game_data.round_data.user2_card.is_some() {
                return RutzoEvent::UserAlreadyChoseACard;
            }
            // Check if the user nft selected exists in his cards selected
            if !Contract::player_contains_selected_card(&game_data.user_2.as_ref().unwrap().nfts_chosen, card_id) {
               return RutzoEvent::NFTSelectedIsNotInCardsSelected(card_id);
            }
            game_data.round_data.user2_card = Some(card_id);
        }
        
        game_data.round_state = MatchState::InProgress;
        
        // It is checked if any user has not chosen their letter
        if game_data.round_data.user1_card.is_none() || game_data.round_data.user2_card.is_none() {
            return RutzoEvent::UserChoseCard;
        }
        
        self.game_with_user(user_current_game)
    }
    
     pub fn game_with_user(&mut self, game_id: GameId) -> RutzoEvent {
        let game_data = self.games2.get_mut(game_id).unwrap();
         
        let Some(user1_card_id) = game_data.round_data.user1_card.as_ref() else {
            return RutzoEvent::UserHasNotYetSelectedACard(
                game_data.user_1.user_id
            );
        };
        
        let Some(user2_card_id) = game_data.round_data.user2_card.as_ref() else {
            return RutzoEvent::UserHasNotYetSelectedACard(
                game_data.user_2.as_ref().unwrap().user_id
            );
        };
        
        let Some(user1_card_data) = game_data
            .user_1
            .nfts_chosen
            .iter()
            .find(|card_data| card_data.nft_token_id == *user1_card_id) else {
                return RutzoEvent::NFTSelectedIsNotInCardsSelected(*user1_card_id);
        };    
        
        let Some(user2_card_data) = game_data
            .user_2 
            .as_ref ()           
            .unwrap()
            .nfts_chosen
            .iter()
            .find(|card_data| card_data.nft_token_id == *user2_card_id) else {
            return RutzoEvent::NFTSelectedIsNotInCardsSelected(*user2_card_id);
        };
        
        let user_1 = game_data.user_1.user_id;
        let user_2 = game_data.user_2.as_ref().unwrap().user_id;
        
        let CardData {
            nft_type: user1_nft_type,
            nft_power: user1_nft_power,
            nft_token_id: user1_chosen_nft,
            ..
        } = user1_card_data;
        
        let CardData {
            nft_type: user2_nft_type,
            nft_power: user2_nft_power,
            nft_token_id: user2_chosen_nft,
            ..
        } = user2_card_data;
        
        let round_result = Contract::start_round(
            user1_nft_type.clone(),
            *user1_nft_power,
            user2_nft_type.clone(),
            *user2_nft_power
        );
        
        let round_state: MatchState;
        
        match round_result {
            MatchResult::PlayerOneWins => {
                round_state = MatchState::RoundFinished { 
                    winner: UserRoundData {
                        user_id: user_1,
                        chosen_nft: *user1_chosen_nft
                    }, 
                    loser: UserRoundData {
                        user_id: user_2,
                        chosen_nft: *user2_chosen_nft
                    }, 
                    draw: false 
                };
                game_data.user1_wins += 1;
            },
            MatchResult::PlayerTwoWins => {
                round_state = MatchState::RoundFinished { 
                    winner: UserRoundData {
                        user_id: user_2,
                        chosen_nft: *user2_chosen_nft
                    }, 
                    loser: UserRoundData {
                        user_id: user_1,
                        chosen_nft: *user1_chosen_nft
                    }, 
                    draw: false 
                };
                game_data.user2_wins += 1;
            },
            MatchResult::Draw => {
                round_state = MatchState::RoundFinished { 
                    winner: UserRoundData {
                        user_id: user_1,
                        chosen_nft: *user1_chosen_nft
                    }, 
                    loser: UserRoundData {
                        user_id: user_2,
                        chosen_nft: *user2_chosen_nft
                    }, 
                    draw: true 
                };
            }
        }
        
        let actual_round = game_data.round;
        game_data.round += 1;
        game_data.round_data.user1_card = None;
        game_data.round_data.user2_card = None;
        game_data.round_state = round_state;
        
        if game_data.round >= 3 && game_data.user1_wins != game_data.user2_wins {
            game_data.match_state = if game_data.user1_wins > game_data.user2_wins {
                self.pending_transfers.insert(user_2, (user_1, game_id));
                MatchState::Finished { 
                    winner: user_1, 
                    loser: user_2 
                }
            } else {
                self.pending_transfers.insert(user_1, (user_2, game_id));
                MatchState::Finished {
                    winner: user_2,
                    loser: user_1 
                }
            };
            
            self.player_finish_game(user_1, game_id);
            self.player_finish_game(user_2, game_id);
            
            return RutzoEvent::MatchFinished(game_id as u64);
        }
        
        RutzoEvent::RoundFinished(actual_round)
    }
    
    pub fn set_player_in_current_game(&mut self, user_id: ActorId, game_id: GameId) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.current_game = Some(game_id);
                user_game_data.recent_past_game = None;
            });
    }
    
    pub fn player_finish_game(&mut self, user_id: ActorId, game_id: GameId) {
        self.games_information_by_user
            .entry(user_id)
            .and_modify(|user_game_data| {
                user_game_data.recent_past_game = Some(game_id);
                user_game_data.current_game = None;
                user_game_data.past_games.push(game_id);
            });
    }
    
    pub fn user_actual_game_id(&self, user_id: ActorId) -> Option<GameId> {
        self.games_information_by_user
            .get(&user_id)
            .unwrap()
            .current_game
    } 
    
    
    
    //pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId, power: u8) -> RutzoEvent {
    pub async fn play_game(&mut self, user_id: ActorId, token_id: TokenId, play_with_bot: bool) -> RutzoEvent {
        if !self.is_register(&user_id) {    
            return RutzoEvent::AccountNotExists(user_id);
        }
            
        /*
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
            */
            
        let answer = Contract::nft_data_by_tokenid(
            self.nft_contract.unwrap(), 
            token_id
        ).await;
                                                            
        if answer.is_err() {
            return RutzoEvent::CommunicationError(self.nft_contract.unwrap());
        }
                                
        if let NFTEvent::TokenIdNotExists(token_id) = answer.clone().unwrap() {
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
        
        let nft_power: Power = nft_data.reference
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
        
        if !play_with_bot {
            self.play_online(user_id, token_id, nft_type, nft_power, nft_data).await
        } else {
            self.play_bot().await
        }
    }
    
    pub async fn play_online(&mut self, user_id: ActorId, token_id: TokenId, nft_type: NFTCardType, nft_power: Power, nft_data: TokenMetadata) -> RutzoEvent {
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
                    (user1_id, user_id, token_id, true)
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
            self.pending_transfers.insert(loser, (winner, game_id));
            if Contract::transfer_match_nft(
                self.nft_contract.unwrap(),
                winner, 
                token_id,
                self.transaction_id
            ).await.is_err() {
                return RutzoEvent::PendingTransfer;
            }
            self.pending_transfers.remove(&loser);
            self.transaction_id = self.transaction_id.saturating_add(1);
        }
        
        RutzoEvent::MatchFinished(game_id as u64)
    }
    
    pub async fn play_bot(&mut self) -> RutzoEvent {
        RutzoEvent::MatchFinished(0)
    }
}

impl From<Contract> for ContractState {
    fn from(value: Contract) -> Self {
        let Contract {
            owner,
            nft_contract,
            games_information_by_user,
            games_waiting,
            // games,
            games2: games,
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
            send_data_to,
            ..
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
            .map(|(user_id, (user_id_winner, game_id))| (*user_id, (*user_id_winner, *game_id as u64)))
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
            //games,
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




