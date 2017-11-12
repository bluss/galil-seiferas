#!/bin/sh

# Run all the fuzzers for a limited number of runs

N=-1
TIME=10
D=$(dirname "$0")

for file in "$D"/run*.sh ; do
	$file -runs=$N -max_total_time=$TIME "$@"
done
