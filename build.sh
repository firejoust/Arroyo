#!/bin/bash

err () {
    echo "ERROR: $1"
    exit 1
}

# clone repository from GitHub
git clone https://github.com/firejoust/arroro && cd arroyo || err "Unable to clone repository, check your internet connection and try again."

# compile release binary with cargo
cargo build --release && cd ./target/release || err "Unable to compile binary, is cargo installed?"

# move binary to directory of execution
mv arroyo $PWD && cd $PWD || err "Failed to move arroyo binary to \"$PWD\"."

# delete source files
rm -Rf arroyo || err "Failed to remove arroyo source files. Have the permissions changed?"

exit 0