#![no_std]
use gstd::{prelude::*, msg, ActorId, Vec, prog::ProgramGenerator, CodeId, exec, BTreeMap};
use program_io::{
    InitContractData,
    RutzoEvent,
    RutzoAction,
    NewPlayerWithIdData,
    GameData,
    GameState,
    ContractData,
    MatchContractData,
    UserData,
};

static mut CONTRACT: Option<ContractData> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitContractData = msg::load()
        .expect("Error in decoding init message 'InitContractData'");
    unsafe {
        CONTRACT = Some(ContractData { 
            owner: msg::source(),
            match_contract: config.match_contract,
            nft_contract: config.nft_contract,
            users: BTreeMap::new(),
            games: BTreeMap::new(),
            transaction_id: 0
        });
    };
}


#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    match action {
        RutzoAction::Login => {
            let user_id = msg::source();
            if !state.users.contains_key(&user_id) {
                msg::reply(RutzoEvent::AccountNotExists(user_id), 0)
                    .expect("Error in sending reply message");
            } else {
                msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
            }         
        },
        RutzoAction::Register => {
            msg::reply(state.register_user(msg::source()).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::PlayGame => {
            let user_id = msg::source();
            if !state.users.contains_key(&user_id) {
                msg::reply(RutzoEvent::AccountNotExists(user_id), 0)
                    .expect("Error in sending message 'RutzoEvent::AccountNotExists(ActorId)'");
                return;
            }
            
            msg::reply(state.join_player_to_game(user_id).await, 0)
                .expect("Error in sending message 'RutzoState'");
        },
        RutzoAction::QuitGame => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::GetCards => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");

        },
        RutzoAction::BuyCard => {
            msg::reply(RutzoEvent::PurchaseSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::EndGame => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");
        },
        /*
        RutzoAction::TransferCard {
            from,
            to,
            matchId
        } => {
            msg::reply(RutzoEvent::LoginSucces, 0).expect("Error in sending reply message");
        },
        RutzoAction::DeleteMainContract => {
            let current_id = msg::source();
            if format!("{:?}", current_id) == MAINACTORID {
                exec::exit(msg::source());
            } else {
                msg::reply(RutzoEvent::LoginSucces, 0)
                    .expect("Error in reply");
            }
        },
        RutzoAction::DeleteMainContractSecond => {
            exec::exit(msg::source());
        },
        RutzoAction::DeleteContracts => {
            state.games
                .iter()
                .for_each(|game| {
                    let delete_contract_message = MatchContractMessage::DeleteContract;
                    msg::send(game.match_id, delete_contract_message, 0)
                        .expect("Error in sending message to match contract");
                });
            msg::reply(RutzoEvent::LoginSucces, 0)
                .expect("Error in reply message");
        }
        */
        _ => {
            msg::reply(RutzoEvent::ReplySuccess, 0).expect("Error in sending reply message");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0).expect("Error in sending reply state");
}



pub fn state_mut() -> &'static mut ContractData {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

pub fn state_ref() -> &'static ContractData {
    let state = unsafe { CONTRACT.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}














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