#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# set environment variables
BASEDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# compile source code
printf "compiling rust source...\n"
su - $(logname) -c "cargo build --manifest-path $BASEDIR/../src/rust/Cargo.toml --release"

# FIXME: Temporarily run the bot from here to see if it works, build the service in /opt/ later
su - et_mm -s /bin/bash -c "$BASEDIR/../src/rust/target/release/et-mm-bot"
