#![allow(unused)]
#![no_std]

use gstd::{prelude::*, msg, ActorId, Clone};
use gear_lib::non_fungible_token::token::TokenId;

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    CardSelected {
        token_id: TokenId,
        actor_id: ActorId
    },
    NewPlayer(ActorId),
    NewPlayerWithMainId(NewPlayerWithIdData)
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    CardDoesNotExist(String),
    FinishSucces
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ConectionState {
    Connected,
    Disconnected
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NewPlayerWithIdData {
    pub main_id: ActorId,
    pub new_player: ActorId
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Scoreboard {
    pub main_contract_id: Option<ActorId>,
    pub score_p1: u8, 
    pub score_p2: u8,
    pub user1_id: Option<ActorId>,
    pub user2_id: Option<ActorId>,

    pub round: u8
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Gameboard {
    pub player1: Player,
    pub player2: Player,
    
    pub game_tie: bool, 
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Player {
    pub player_id: ActorId,
    pub connection_state: ConectionState,
    // pub deck: Vec<TokenId>,
    pub winner: bool,
}

static mut GAME_SCORE: Option<Scoreboard> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe { 
        GAME_SCORE = Some(Scoreboard {
            main_contract_id: None,
            score_p1: 0, 
            score_p2: 0,
            user1_id: None,
            user2_id: None,
            round: 1
        });
    };
}

#[no_mangle]
extern "C" fn handle() {
    let message = msg::load::<InputMessages>().expect("Failed to load message");
    let mut player1: Player;
    let mut player2: Player;
    
    match message {
        InputMessages::CardSelected { token_id, actor_id } => {
            msg::reply(String::from("User select a card"), 0).expect("Error in reply message: Card selected");
            /* let mut game_score = unsafe {
                Gameboard {player1, player2, game_tie: false};
            };

            
            if game_score.player1 != null {
                player1 = Player { player_id: actor_id, winner: false };
            } else if game_score.player2 != null {
                player2 = Player { player_id: actor_id, winner: false };
            }
            // Verificar si la carta existe

            // Registrar la carta seleccionada por el jugador actual
            if game_score.player1.card_selected == true {
                // Jugador 1 seleccionó una carta
                game_score.player1.card_selected = true;
            } else if actor_id == game_score.player2.player_id {
                // Jugador 2 seleccionó una carta
                game_score.player2.card_selected = true;
            }

            // Verificar si ambos jugadores han seleccionado una carta
            if game_score.player1.card_selected && game_score.player2.card_selected {
                // Ambos jugadores han seleccionado una carta, continuar con el juego

                // Realizar las operaciones correspondientes según el resultado de la partida
                if game_score.round == 3 {
                    // Última ronda, enviar mensaje de finalización
                    msg::send(actor_id, "Game over.", 0).expect("Failed to send message");

                    // Registrar puntajes, determinar ganador, etc.
                    // ...

                } else {
                    // Aún no es la última ronda, seguir con el juego

                    // Registrar puntajes, determinar ganador, etc.
                    // ...

                    // Incrementar la ronda
                    game_score.round += 1;

                    // Reiniciar la selección de cartas para la siguiente ronda
                    game_score.player1.card_selected = false;
                    game_score.player2.card_selected = false;
                }
            } */
        },
        InputMessages::NewPlayerWithMainId(new_player_in_game_and_main_id) => {
            unsafe {
                if let Some(data) = &mut GAME_SCORE {
                    if let None = data.user1_id {
                        data.user1_id = Some(new_player_in_game_and_main_id.new_player);
                    };
                    data.main_contract_id = Some(new_player_in_game_and_main_id.main_id);
                }
            };
            msg::reply("Match created", 0).expect("Error in reply message: New player with main id");
        },
        InputMessages::NewPlayer(new_player_in_game) => {
            unsafe {
                if let Some(data) = &mut GAME_SCORE {
                    if let None = data.user2_id {
                        data.user2_id = Some(new_player_in_game);
                    };
                }
            };
        }
    }
}


#[no_mangle]
extern "C" fn handle_reply() {}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

extern "C" fn state() {
    let state_result = unsafe {
        if let Some(data) = &GAME_SCORE {
            Some(data.clone())
        }
        else {
            None
        }
    };
    msg::reply(state_result.unwrap(), 0).expect("Error in reply state");
}

fn player_deck_select() {}

fn player_card_selected() {}