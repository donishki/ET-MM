#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# set environment variables
BASEDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# prepare setup scripts for execution
printf "Preparing setup scripts..."
chmod +x $BASEDIR/setup/environment.sh
chmod +x $BASEDIR/setup/database.sh
printf "\t[OK]\n"

# execute environment setup script
printf "Executing environment setup script...\n"
/bin/bash $BASEDIR/setup/environment.sh

printf "Executing database setup script...\n"
/bin/bash $BASEDIR/setup/database.sh
