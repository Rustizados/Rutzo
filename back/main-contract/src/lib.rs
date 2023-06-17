#![no_std]
use gear_lib::non_fungible_token::token::TokenId;
use gstd::{prelude::*, msg, ActorId};

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
    RegisterSucces,
    LoginSucces,
    PurchaseSucces,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum id_type {
    user_id,
    game_id
}

#[derive(Encode, Decode, TypeInfo)]
pub struct user_games_id {
    pub users: Vec<ActorId>,
    pub games: Vec<ActorId>
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
        InputMessages::Login(actor_id) => {
            if let id_type::user_id = actor_id {
                let ids_list = unsafe {
                    USERSANDGAMESID.users
                        .as_mut()
                        .expect("The contract is no initialized");
                };
                let id = msg::source();
                let user_account = ids_list
                    .into_iter()
                    .find(|&account| account == id);
                if let None = user_account {
                    user_account.push(id);
                }
            }
        },
        InputMessages::Register(actor_id) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::PlayGame(actor_id) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::QuitGame(actor_id) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::GetCards(actor_id) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::BuyCard(TokenId) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
        },
        InputMessages::EndGame(actor_id) => {
            msg::reply(Answer::LoginSucces, 0).expect("Error in sending reply message");
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
extern "C" fn handle_reply() {}