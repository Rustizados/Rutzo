use main_contract_io::MainContractMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<MainContractMetadata>();
}