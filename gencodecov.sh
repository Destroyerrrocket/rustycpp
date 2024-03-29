#!/bin/sh
RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test --profile codecoverage
mkdir -p target/coverage
grcov . --binary-path ./target/codecoverage/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov
grcov . --binary-path ./target/codecoverage/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/

rm cargo-test-*.profraw