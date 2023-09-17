use gtest::{Log, System, Program};
use program_io::*;

#[test]
fn contract_init() {
    let sys = System::new();
    init_contract(&sys);
    let contract = sys.get_program(1);
}

fn init_contract(sys: &System) {
    sys.init_logger();
    let contract = Program::current(&sys);
    let res = contract.send(2, "");
    assert!(!res.main_failed());
}