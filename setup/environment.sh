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

# add bot service user
printf "Adding et_mm user..."
useradd -r -s /bin/false et_mm
printf "\t[OK]\n"

# update package repository
printf "Updating system package repository...\n"
yum update -y

# install runtime software
if [ "$LANGUAGE" == "java" ]; then
	# install OpenJDK
    printf "Installing OpenJDK 11...\n"
    yum install -y java-11-openjdk
elif [ ! -z "$BUILD" ]; then
	# install rustup
	printf "Installing Rustup...\n"
	curl --proto '=https' --tlsv1.2 -sSf -o /tmp/sh.rustup.rs https://sh.rustup.rs
	chmod +x /tmp/sh.rustup.rs
	su - $(logname) -c "/tmp/sh.rustup.rs -y"
	su - $(logname) -c "source $HOME/.cargo/env"
	# cleanup
	rm /tmp/sh.rustup.rs
fi

# install PostgreSQL
printf "Installing PostgreSQL...\n"
yum install -y postgresql-server

# initialize database
printf "Creating PostgreSQL service:\n"
postgresql-setup --initdb --unit postgresql
systemctl start postgresql
systemctl enable postgresql
