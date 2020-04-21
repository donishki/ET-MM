#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# add bot service user
printf "Adding et_mm user..."
useradd -r -s /bin/false et_mm
printf "\t[OK]\n"

# update package repository
printf "Updating system package repository...\n"
yum update -y

# install OpenJDK 11
printf "Installing OpenJDK 11...\n"
yum install -y java-11-openjdk

# install PostgreSQL 
printf "Installing PostgreSQL...\n"
yum install -y postgresql-server

# initialize database
printf "Creating PostgreSQL service:\n"
postgresql-setup --initdb --unit postgresql
systemctl start postgresql
systemctl enable postgresql
