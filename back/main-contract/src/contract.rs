#![no_std]
use gstd::{prelude::*, msg, ActorId, Vec, prog::ProgramGenerator, CodeId, exec};
use program_io::{
    InputMessages,
    Answer,
    NewPlayerWithIdData,
    GameData,
    GameState,
    UserGamesId,
    MatchContractData,
    UserData,
    MatchContractMessage
};
use hex_literal::hex;

const MAINACTORID: &str = "0x52bb42ac24f528ab6e78b2bcf1afdc26cdd5b5585a544266884fc40705903d5c";

static mut USERSANDGAMESID: Option<UserGamesId> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        USERSANDGAMESID = Some(UserGamesId { 
            users: Vec::new(),
            games: Vec::new(),
        });
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    match action {
        InputMessages::Login => {
            let id = msg::source();
            let user_account = account_from_vec(id);
            if let None = user_account {
                msg::reply(Answer::AccountNotExists(String::from("The account is not registered")), 0)
                    .expect("Error in sending reply message");
            } else {
                msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
            }            
        },
        InputMessages::Register => {
            let id = msg::source();
            let user_account = account_from_vec(id);
            if let None = user_account {
                add_account_to_vec(id);
                msg::reply(Answer::RegisterSucces, 0).expect("Error in sending reply message");
            } else {
                msg::reply(Answer::AccountAlreadyExist(String::from("The account is already registered")), 0)
                    .expect("Error in sending reply message");
            }
        },
        InputMessages::PlayGame => {
            let game_to_join = search_game_waiting();
            match game_to_join {
                Some(game_waiting) => {
                    let current_user_id = msg::source();
                    let new_user = MatchContractMessage::NewUser(current_user_id);
                    msg::send(game_waiting.actor_game_id, new_user, 0).expect("Unable to send message to new contract");
                    msg::reply(game_waiting.actor_game_id, 0).expect("Error sending reply");
                },
                None => {
                    let submitted_code: CodeId =
                        hex_literal::hex!("aa7b8462984cba48bab6bce9b599caea61d914cbcb9682d654e4709b2abbcd3a")
                            .into();
                    let main_contract_id = exec::program_id();
                    let current_user_id = msg::source();
                    let match_data = MatchContractData {
                        first_user: Some(current_user_id),
                        second_user: None,
                        mainContractId: main_contract_id
                    };
                    let (message_id, program_id) = ProgramGenerator::create_program_with_gas(
                        submitted_code, 
                        match_data.encode(), 
                        //30_513_631_315
                        11_000_000_000,
                        0
                    )
                    .unwrap();
                    for user in state.users.iter_mut() {
                        if user.user_id == current_user_id {
                            user.in_game = true;
                            break;
                        }
                    }
                    add_new_game_to_vec(program_id.clone());
                    msg::send(program_id, match_data, 0).expect("Unable to send message to new contract");
                    msg::reply(String::from("Match created!"), 0)
                        .expect("Error in sending reply message");
                }
            }
        },
        InputMessages::QuitGame => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::GetCards => {
            msg::reply(Answer::ReplySuccess, 0).expect("Error in sending reply message");

        },
        InputMessages::BuyCard => {
            msg::reply(Answer::PurchaseSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::EndGame => {
            msg::reply(Answer::ReplySuccess, 0).expect("Error in sending reply message");
        },
        InputMessages::TransferCard {
            from,
            to,
            matchId
        } => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::DeleteMainContract => {
            let current_id = msg::source();
            if format!("{:?}", current_id) == MAINACTORID {
                exec::exit(msg::source());
            } else {
                msg::reply(String::from("user can't delete contract: {current_id}"), 0)
                    .expect("Error in reply");
            }
        },
        InputMessages::DeleteMainContractSecond => {
            exec::exit(msg::source());
        },
        InputMessages::DeleteContracts => {
            state.games
                .iter()
                .for_each(|game| {
                    let delete_contract_message = MatchContractMessage::DeleteContract;
                    msg::send(game.actor_game_id, delete_contract_message, 0)
                        .expect("Error in sending message to match contract");
                });
            msg::reply(String::from("All match contracts deleted!"), 0)
                .expect("Error in reply message");
        }
    }
}

#[no_mangle]
extern  "C" fn handle_reply() {
    let response: MatchContractMessage = msg::load().expect("Error in handle reply message");
    let state = state_mut();
    match response {
        MatchContractMessage::AcceptedUser => {
            let match_id = msg::source();
            for game in state.games.iter_mut() {
                if game.actor_game_id == match_id {
                    game.actual_state = GameState::GameInProgress;
                    break;
                }
            }
        },
        MatchContractMessage::MatchResult(match_result) => {
            
        },
        MatchContractMessage::ErrorAddingUser((ActorId, String)) => {
            
        },
        _ => {}
    }
}

#[no_mangle]
extern "C" fn state() {
    /*
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
    let games_vec: Vec<GameData> = unsafe {
        if let Some(vect) = &USERSANDGAMESID {
            vect.games
                .iter()
                .map(|game_in_vec| GameData {
                    actor_game_id: game_in_vec.actor_game_id.clone(),
                    actual_state: if let GameState::waiting_for_player = game_in_vec.actual_state {
                        GameState::waiting_for_player
                    } else {
                        GameState::game_in_progress
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    };
    let vectors = UserGamesId {
        users: users_vec,
        games: games_vec
    };
    */
    msg::reply(state_mut(), 0).expect("Error in sending reply state");
}

fn state_mut() -> &'static mut UserGamesId {
    let state = unsafe { USERSANDGAMESID.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static UserGamesId {
    let state = unsafe { USERSANDGAMESID.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn account_from_vec(user_id: ActorId) -> Option<&'static UserData> {
    let state = state_ref();
    state.users
        .iter()
        .find(|user| user.user_id == user_id)
}

fn add_account_to_vec(user_id: ActorId) {
    let state = state_mut();
    state.users.push(
        UserData {
            user_id,
            in_game: false
        }
    );
}

fn game_id_is_in_vec(game_data_to_find_id: ActorId) -> Option<&'static GameData> {
    let state = state_ref();
    state.games
        .iter()
        .find(|actual_game| actual_game.actor_game_id == game_data_to_find_id)
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

fn add_new_game_to_vec(actor_game_id: ActorId) {
    let state = state_mut();
    state.games.push(
        GameData {
            actor_game_id,
            actual_state: GameState::WaitigForPlayer
        }
    );
}