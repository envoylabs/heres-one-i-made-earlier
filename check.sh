#!/bin/sh

set -e

cargo clippy --all-targets -- -D warnings
cargo fmt
RUST_BACKTRACE=full cargo unit-test
cargo schema
