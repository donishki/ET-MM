#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# process arguments
unset BUILD LANGUAGE
while getopts ":bl:" OPTIONS; do
	case "${OPTIONS}" in
        b)
			BUILD=1
			;;
        l)
			LANGUAGE=${OPTARG}
			;;
        *)
            printf "Usage: ./environment.sh -b (optional: compile source) -l (required: desired bot language) <java or rust>\n"
            exit 1
            ;;
    esac
done
if [ "$LANGUAGE" != "java" ] && [ "$LANGUAGE" != "rust" ]; then
    printf "Usage: ./environment.sh -b (optional: compile source) -l (required: desired bot language) <java or rust>\n"
    printf "\tInvalid option for -l: $LANGUAGE. Only Java and Rust are supported.\n"
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
if [ ! -z "$BUILD" ]; then
	/bin/bash $BASEDIR/setup/environment.sh -b -l $LANGUAGE
else
	/bin/bash $BASEDIR/setup/environment.sh -l $LANGUAGE
fi

# execute database setup script
printf "Executing database setup script...\n"
/bin/bash $BASEDIR/setup/database.sh

# compile application
if [ ! -z "$BUILD" ]; then
    if [ "$LANGUAGE" == "rust" ]; then
        /bin/bash $BASEDIR/setup/compile_rust.sh
    fi
fi
