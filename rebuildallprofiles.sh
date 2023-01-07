#!/bin/sh
cargo clean
RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo build --tests --profile codecoverage
cargo build
cargo build --release
cargo build --profile test
cargo build --profile bench
