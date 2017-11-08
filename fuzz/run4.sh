#!/bin/sh

# Fuzzing works fine on this particular nightly

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
# -a for debug asserts
cargo +$V fuzz run -O -a debug_asserts -- -max_len=256 "$@"
