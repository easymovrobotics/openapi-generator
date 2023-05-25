gen specs="openapi.yaml" output="./output" +OPTIONS="":
    RUST_LOG=info cargo run -- templates/rust {{specs}} -d {{output}} {{OPTIONS}}
    cargo fmt --manifest-path output/Cargo.toml

check:
    (cd ./output && cargo check --all-features)

watch directory="output":
    cd {{directory}} && cargo watch -x fmt -x "check --all-features --all-targets"
