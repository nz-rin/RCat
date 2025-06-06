#!/usr/bin/env bash

echo "COMPILING"
rustc rcat.rs -o rcat

if [[ $? != 0 ]]; then
	echo "FAILED"
else
	echo "DONE"
fi
