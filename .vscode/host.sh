#!/bin/bash

cargo_script=cargo_script

mkfifo $cargo_script

trap "rm $cargo_script" EXIT

while read test < $cargo_script; do
clear
    cargo clippy
done

