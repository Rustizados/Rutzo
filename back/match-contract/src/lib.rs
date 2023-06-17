#![allow(unused)]
#![no_std]

use gstd::{prelude::*, msg, ActorId};
use gear_lib::{non_fungible_token::token::TokenId};

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    // struct idwey y cartas
    CardSelected {
        token_id: TokenId,
        actor_id: ActorId
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Answer {
    CardDoesNotExist(String),
    FinishSucces
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Scoreboard {
    pub score_p1: u8, 
    pub score_p2: u8,
    pub round: u8
}

#[derive(Encode, Decode, TypeInfo)]
pub struct GameFinishedSt {
    pub winner: ActorId, 
    pub losser: ActorId, 
    pub game_status: GameState
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Gameboard {
    pub player1: ActorId,
    pub player2: ActorId  
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameState {
    Tied,
    Won
}

#[derive(Default)]
pub struct PlayerState {
    pub card_selected: bool,
}

static mut GAME_SCORE: Option<Scoreboard> = None;
static mut PLAYERS: Option<[ActorId; 2]> = None;
static mut PLAYER_1_STATE: PlayerState = PlayerState::default();
static mut PLAYER_2_STATE: PlayerState = PlayerState::default();

#[no_mangle]
extern "C" fn init() {
    unsafe { 
        GAME_SCORE = Some(Scoreboard { 
            score_p1: 0,
            score_p2: 0,
            round: 1 
        });

        let array: [ActorId; 2] = [ActorId::new([1; 32]), ActorId::new([2; 32])];

        PLAYERS = Some(array);
    };
}

#[no_mangle]
extern "C" fn handle() {
    let message = msg::load::<InputMessages>().expect("Failed to load message");
    match message {
        InputMessages::CardSelected {token_id, actor_id} => {
            if actor_id == ActorId::new([1; 32]) {
                unsafe {
                    PLAYER_1_STATE.card_selected = true;
                }
            } else if actor_id == ActorId::new([2; 32]) {
                unsafe {
                    PLAYER_2_STATE.card_selected = true;
                }
            }

            // Verificar si ambos jugadores han seleccionado una carta
            let players_ready = unsafe {
                PLAYER_1_STATE.card_selected && PLAYER_2_STATE.card_selected
            };
            
            if players_ready {
                // Actualizar el marcador y la ronda
                unsafe {
                    if let Some(scoreboard) = &mut GAME_SCORE {
                        // Realizar lógica de juego y actualizar el marcador según las cartas seleccionadas
                        // Puedes usar las variables `token_id`, `PLAYER_1_STATE` y `PLAYER_2_STATE`
                        // para determinar las cartas seleccionadas por cada jugador y actualizar el marcador
                        // Aquí se asume que la lógica del juego ya ha sido implementada
                        // y se incrementa el marcador del jugador 1 en 1 por cada carta seleccionada
                        scoreboard.score_p1 += 1;

                        // Verificar si se completaron las tres rondas
                        if scoreboard.round == 3 {
                            // Enviar mensaje de finalización del juego
                            /* let game_finished = GameFinished {
                                winner: ActorId(1), // Ejemplo: jugador 1 es el ganador
                                loser: ActorId(2),  // Ejemplo: jugador 2 es el perdedor
                                game_status: GameState::Won,
                            }; */
                            // msg::reply(&game_finished, 0).expect("Failed to send game finish message");
                        } else {
                            // Reiniciar el estado de los jugadores para la próxima ronda
                            PLAYER_1_STATE.card_selected = false;
                            PLAYER_2_STATE.card_selected = false;
                            // Incrementar la ronda para la próxima ronda
                            scoreboard.round += 1;
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn handle_reply() {}

fn player_deck_select() {}

fn player_card_select() {
    let message: InputMessages = msg::load().expect("Invalid message");

    /* match message {
        InputMessages::CardSelected(token_card) => {
            
            // msg::send(main_id, String::from("Card selected successfully"), 0).expect("Can't send a `SendHelloTo` message");
        }
        __ => {}
    } */

}