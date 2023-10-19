#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut, In};
use gear_lib::non_fungible_token::token::TokenId;

pub struct ProgramMetadata;

pub const MATCH_CODEID = "aa7b8462984cba48bab6bce9b599caea61d914cbcb9682d654e4709b2abbcd3a";
pub const GAS_FOR_CREATION_MATCH = 11_000_000_000;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<RutzoAction, RutzoEvent>;
    type Reply = In<MatchContractMessage>;
    type Others = ();
    type Signal = ();
    type State = Out<ContractData>;
}

// State: Beta -------------------------
#[derive(Encode, Decode, TypeInfo)]
pub enum MatchContractMessage {
    NewUser(ActorId),
    MatchResult(MatchContractResults),
    AcceptedUser,
    DeleteContract,
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
    AccountAlreadyExist(String),
    AccountNotExists(String),
    QueryNotAllowed(String),
    RegisterSucces,
    LoginSucces,
    PurchaseSucces,
    ReplySuccess,
    NewPlayer(ActorId),
    NewPlayerWithMainId(NewPlayerWithIdData),
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
pub struct ContractData {
    pub users: Vec<UserData>,
    pub games: Vec<GameData>,
}

impl ContractData {
    pub 
    
    pub async fn join_player_to_game(user_id: &ActorId) -> Result<(), String>{
        let user_info = match self.user_data(user_id) {
            Some(user_data) => user_data,
            None => return Err(String::from("User is no registered"))
        };
        
        if user_info.in_game {
            return Err(String::from("User is already in a match"));
        }
        
        user_info.in_game = true;
        
        
        
        
        add_new_game_to_vec(program_id.clone());
        msg::send(program_id, match_data, 0).expect("Unable to send message to new contract");
        msg::reply(String::from("Match created!"), 0)
            .expect("Error in sending reply message");
    }
    
    async fn find_match_waiting_player(&self) -> &ActorId {
        let match_data = self.games
            .iter()
            .find(|&actual_game| 
                if let GameState::WaitigForPlayer = actual_game.actual_state {
                    true
                } else {
                    false
                }
            );
        match match_data {
            Some(data) => data.match_id,
            None => self.create_match().await
        }
    }
    
    async fn create_match(&mut self) -> ActorId {
        let submitted_code: CodeId =hex_literal::hex!(MATCH_CODEID).into();
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
        
        self.games.push(
            GameData {
                match_id.clone(),
                actual_state: GameState::WaitigForPlayer
            }
        ); 
        
        match_id
    }
    
    pub fn user_data(&self, user_id: &ActorId) -> Option<&'static mut GameData> {
        state.users
            .iter_mut()
            .find(|user| user.user_id == user_id)
    }
    
    pub fn user_is_in_match(&self, user_id: &ActorId) -> Result<bool, ()> {
        let user_data = self.users
            .iter()
            .find(|user| user.user_id == user_id);
        
        match user_data {
            Some(data) => Ok(data.in_game),
            None => Err(())
        }
    }
}



fn game_id_is_in_vec(game_data_to_find_id: ActorId) -> Option<&'static GameData> {
    let state = state_ref();
    state.games
        .iter()
        .find(|actual_game| actual_game.match_id == game_data_to_find_id)
}

fn search_game_waiting() -> Option<&'static GameData> {
    let state = state_ref();
    state.games
        .iter()
        .find(|&actual_game| 
            if let GameState::WaitigForPlayer = actual_game.actual_state {
                true
            } else {
                false
            }
        )
}


