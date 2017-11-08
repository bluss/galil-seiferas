#!/bin/sh

# Fuzzing works fine on this particular nightly

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run -O fuzz_target_3 -- -max_len=512 -only_ascii=1 "$@"
