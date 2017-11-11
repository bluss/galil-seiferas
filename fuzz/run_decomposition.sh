#!/bin/sh

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run -O -a decomposition -- -max_len=512 "$@"
