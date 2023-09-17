use gear_wasm_builder::WasmBuilder;
use program_io::ProgramMetadata;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(ProgramMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}