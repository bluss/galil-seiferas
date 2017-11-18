#!/bin/sh

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run -O -a known_substring_i16 -- -max_len=1024 "$@"
