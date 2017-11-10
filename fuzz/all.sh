#!/bin/sh

# Run all the fuzzers for a limited number of runs

N=250000
D=$(dirname "$0")

for file in "$D"/run*.sh ; do
	$file -runs=$N
done
