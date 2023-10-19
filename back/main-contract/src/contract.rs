#![no_std]
use gstd::{prelude::*, msg, ActorId, Vec, prog::ProgramGenerator, CodeId, exec};
use program_io::{
    InputMessages,
    RutzoEvent,
    NewPlayerWithIdData,
    GameData,
    GameState,
    ContractData,
    MatchContractData,
    UserData,
    MatchContractMessage
};
use hex_literal::hex;

const MAINACTORID: &str = "0x52bb42ac24f528ab6e78b2bcf1afdc26cdd5b5585a544266884fc40705903d5c";

static mut CONTRACT_DATA: Option<ContractData> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        CONTRACT_DATA = Some(ContractData { 
            users: Vec::new(),
            games: Vec::new(),
        });
    };
}


#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    match action {
        InputMessages::Login => {
            let id = msg::source();
            let user_account = account_from_vec(id);
            if let None = user_account {
                msg::reply(RutzoEvent::AccountNotExists(String::from("The account is not registered")), 0)
                    .expect("Error in sending reply message");
            } else {
                msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
            }            
        },
        InputMessages::Register => {
            let id = msg::source();
            let user_account = account_from_vec(id);
            if let None = user_account {
                add_account_to_vec(id);
                msg::reply(RutzoEvent::RegisterSucces, 0).expect("Error in sending reply message");
            } else {
                msg::reply(RutzoEvent::AccountAlreadyExist(String::from("The account is already registered")), 0)
                    .expect("Error in sending reply message");
            }
        },
        InputMessages::PlayGame => {
            let game_to_join = search_game_waiting();
            match game_to_join {
                Some(game_waiting) => {
                    let current_user_id = msg::source();
                    let new_user = MatchContractMessage::NewUser(current_user_id);
                    msg::send(game_waiting.match_id, new_user, 0).expect("Unable to send message to new contract");
                    msg::reply(game_waiting.match_id, 0).expect("Error sending reply");
                },
                None => {
                    
                    
                    
                    
                }
            }
        },
        InputMessages::QuitGame => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::GetCards => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");

        },
        InputMessages::BuyCard => {
            msg::reply(RutzoEvent::PurchaseSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::EndGame => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");
        },
        InputMessages::TransferCard {
            from,
            to,
            matchId
        } => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
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
                    msg::send(game.match_id, delete_contract_message, 0)
                        .expect("Error in sending message to match contract");
                });
            msg::reply(String::from("All match contracts deleted!"), 0)
                .expect("Error in reply message");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0).expect("Error in sending reply state");
}

fn state_mut() -> &'static mut ContractData {
    let state = unsafe { CONTRACT_DATA.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static ContractData {
    let state = unsafe { CONTRACT_DATA.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}



account_from_vec



// Arbol de analisis sintactico donde las hojas quedan terminales
// y en los nodos internos quedan no terminales.
// Analizador sintactica predictivo.

    /*
    let users_vec: Vec<ActorId> = unsafe {
        if let Some(vect) = &CONTRACT_DATA {
            vect.users
                .iter()
                .map(|&account| account.clone())
                .collect()
        } else {
            Vec::new()
        }
    };
    let games_vec: Vec<GameData> = unsafe {
        if let Some(vect) = &CONTRACT_DATA {
            vect.games
                .iter()
                .map(|game_in_vec| GameData {
                    match_id: game_in_vec.match_id.clone(),
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
    let vectors = ContractData {
        users: users_vec,
        games: games_vec
    };
    */




    
/*
#[no_mangle]
extern "C" fn handle() {
    
}
*/


/*
#[no_mangle]
extern  "C" fn handle_reply() {
    let response: MatchContractMessage = msg::load().expect("Error in handle reply message");
    let state = state_mut();
    match response {
        MatchContractMessage::AcceptedUser => {
            let match_id = msg::source();
            for game in state.games.iter_mut() {
                if game.match_id == match_id {
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
*/