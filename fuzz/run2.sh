#!/bin/sh

# Fuzzing works fine on this particular nightly

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run -O fuzz_target_2 -- -max_len=128 "$@"
