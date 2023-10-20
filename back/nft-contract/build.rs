use gear_wasm_builder::WasmBuilder;
use program_io::NFTMetadata;
use gmeta::Metadata;

fn main() {
    WasmBuilder::with_meta(NFTMetadata::repr())
        .exclude_features(["binary-vendor"])
        .build();
}