#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*};
use program_io::ProgramMetadata;

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn data(state: State) -> String {
        String::from("test");
    }
}