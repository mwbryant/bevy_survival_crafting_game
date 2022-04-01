#!/bin/bash

cargo_script=cargo_script

if   ! test -e $cargo_script 
then
    echo "no host"
    x-terminal-emulator -e bash host.sh
fi

cargo fmt

echo "clippy" > $cargo_script
