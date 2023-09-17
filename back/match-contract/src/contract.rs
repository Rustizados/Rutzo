#![no_std]
use gstd::{prelude::*, msg, exec};
use program_io::{
    MatchContractData,
    MatchContractResults,
    InputMessages,
    MatchContractMessage,
};

static mut STATE: Option<MatchContractData> = None;

#[no_mangle]
extern "C" fn init() {
    let init_message: MatchContractData = msg::load()
        .expect("Error in load message");
    let state = state_mut();
    unsafe {
        STATE = Some(init_message)
    }
}

#[no_mangle]
extern "C" fn handle() {
    let message: MatchContractMessage = msg::load()
        .expect("Error in load message");
    let state = state_mut();
    match message {
        MatchContractMessage::NewUser(user_id) => {
            let current_id = msg::source();
            if current_id == state.mainContractId {
                state.second_user = Some(user_id);
                msg::reply(MatchContractMessage::AcceptedUser, 0)
                    .expect("Error in reply");
            } else {
                msg::reply(
                    MatchContractMessage::QueryNotAllowed(String::from("Actor with id {current_id} can not perform 'NewUser' action")),
                    0
                ).expect("Error in reply message");
            }
        },
        MatchContractMessage::DeleteContract => {
            exec::exit(msg::source());
        },
        _ => {}
    }
    msg::reply(state_mut(), 0).expect("Error in reply message");
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0).expect("Error sending state");
}

fn state_mut() -> &'static mut MatchContractData {
    let state = unsafe { STATE.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}


