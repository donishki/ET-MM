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

# install the service
# FIXME: eventually this is where the service will be setup
#        for now just run the bot
mkdir /opt/et-mm-bot
cp $BASEDIR/../runtime/config.cfg /opt/et-mm-bot/config.cfg
cp $BASEDIR/../src/rust/target/release/et-mm-bot /opt/et-mm-bot/et-mm-bot
chown -R et_mm:et_mm /opt/et-mm-bot
