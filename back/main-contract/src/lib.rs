#![no_std]
use gear_lib::non_fungible_token::token::TokenId;
use gstd::{prelude::*, msg, ActorId, Vec, Clone, prog::ProgramGenerator, CodeId, exec};
use hex_literal::hex;

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    Login(id_type),
    Register(id_type),
    PlayGame(id_type),
    QuitGame(id_type),
    GetCards(id_type),
    BuyCard(TokenId),
    EndGame(id_type),
    TransferCard {
        from: ActorId,
        to: ActorId,
        matchId: ActorId
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    ErrorBuying(String),
    AccountAlreadyExist(String),
    AccountNotExists(String),
    QueryNotAllowed(String),
    RegisterSucces,
    LoginSucces,
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

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct game_data {
    pub actor_game_id: ActorId,
    pub actual_state: game_state
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum game_state {
    waiting_for_player,
    game_in_progress
}

#[derive(Encode, Decode, TypeInfo)]
pub struct user_games_id {
    pub users: Vec<ActorId>,
    pub games: Vec<game_data>
}

static mut USERSANDGAMESID: Option<user_games_id> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        USERSANDGAMESID = Some(user_games_id {
            users: Vec::new(),
            games: Vec::new()
        });
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action = msg::load().expect("Error in loading message");
    match action {
        InputMessages::Login(actor_id_type) => {
            if let id_type::user_id = actor_id_type {
                let id = msg::source();
                let user_account: Option<ActorId> = account_from_vec(id);
                
                if let None = user_account {
                    msg::reply(Answer::AccountNotExists(String::from("The account is no registered")), 0)
                    .expect("Error in sending reply message");
                } else {
                    msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
                }
            } else {
                msg::reply(Answer::QueryNotAllowed(String::from("Id type cannot perform this query.")), 0)
                    .expect("Error in sending reply message");
            }
        },
        InputMessages::Register(actor_id_type) => {
            if let id_type::user_id = actor_id_type {
                let id = msg::source();
                let user_account: Option<ActorId> = account_from_vec(id);
                
                if let None = user_account {
                    add_account_to_vec(id);
                    msg::reply(Answer::RegisterSucces, 0).expect("Error in sending reply message");
                } else {
                    msg::reply(Answer::AccountNotExists(String::from("The is already registered")), 0)
                    .expect("Error in sending reply message");
                }
            } else {
                msg::reply(Answer::QueryNotAllowed(String::from("Id type cannot perform this query.")), 0)
                    .expect("Error in sending reply message");
            }
        },
        InputMessages::PlayGame(actor_id_type) => {
            if let id_type::user_id = actor_id_type {
                let game_to_join = search_game_waiting();
                match game_to_join {
                    Some(game_waiting) => {
                        let current_user_id = msg::source();
                        msg::send(game_waiting.actor_game_id, current_user_id, 0).expect("Unable to send message to new contract");
                    },
                    None => {
                        let submitted_code: CodeId =
                            hex_literal::hex!("263894ffc57841748b2a6a29bea8fe821c15fe6772a8182a0308211150b04cc9")
                                .into();
                        let main_contract_id = exec::program_id();
                        let current_user_id = msg::source();
                        let new_player_with_program_id = NewPlayerWithIdData {
                            main_id: main_contract_id,
                            new_player: current_user_id
                        };
                        let program_id =  ProgramGenerator::create_program_with_gas(submitted_code, b"payload", 10_000_000_000, 0).unwrap();
                        msg::send(program_id, new_player_with_program_id, 0).expect("Unable to send message to new contract");
                    }
                }
            } else {
                msg::reply(Answer::QueryNotAllowed(String::from("Id type cannot perform this query.")), 0)
                    .expect("Error in sending reply message");
            }
        },
        InputMessages::QuitGame(actor_id_type) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::GetCards(actor_id_type) => {
            msg::reply(Answer::ReplySuccess, 0).expect("Error in sending reply message");

        },
        InputMessages::BuyCard(TokenId) => {
            msg::reply(Answer::PurchaseSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::EndGame(actor_id_type) => {
            match actor_id_type {
                id_type::user_id => {

                },
                id_type::game_id => {

                }
            }
        },
        InputMessages::TransferCard {
            from,
            to,
            matchId
        } => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let users_vec: Vec<ActorId> = unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.users
                .iter()
                .map(|&account| account.clone())
                .collect()
        } else {
            Vec::new()
        }
    };
    let games_vec: Vec<game_data> = unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.games
                .iter()
                .map(|game_in_vec| game_data {
                    actor_game_id: game_in_vec.actor_game_id.clone(),
                    actual_state: if let game_state::waiting_for_player = game_in_vec.actual_state {
                        game_state::waiting_for_player
                    } else {
                        game_state::game_in_progress
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    };
    let vectors = user_games_id {
        users: users_vec,
        games: games_vec
    };
    msg::reply(vectors, 0).expect("Error in sending reply state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn account_from_vec(account: ActorId) -> Option<ActorId> {
    unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.users
                .iter()
                .find(|&&user_account| user_account == account)
                .copied()
        } else {
            None
        }
    }
}

fn add_account_to_vec(account: ActorId) {
    unsafe {
        if let Some(vect) = &mut USERSANDGAMESID {
            vect.users.push(account);
        };
    };
}

fn game_id_is_in_vec(game_data_to_find_id: game_data) -> Option<game_data> {
    unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.games
                .iter()
                .find(|&actual_game| actual_game.actor_game_id == game_data_to_find_id.actor_game_id)
                .cloned()
        } else {
            None
        }
    }
}

fn search_game_waiting() -> Option<game_data> {
    let game_waiting: Option<game_data> = unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.games
                .iter()
                .find(|&actual_game| if let game_state::waiting_for_player = actual_game.actual_state {
                    true
                } else {
                    false
                })
                .cloned()
        } else {
            None
        }
    };
    game_waiting
}

fn add_new_game_to_vec(actor_game_id: ActorId) {
    unsafe {
        if let Some(vect) = &mut USERSANDGAMESID {
            vect.games.push(game_data {
                actor_game_id,
                actual_state: game_state::waiting_for_player
            });
        }
    }
}
