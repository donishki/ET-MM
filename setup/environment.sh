#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# add bot service user
printf "adding et_mm user..."
useradd -r -s /bin/false et_mm
printf "\t[OK]\n"

# update package repository
printf "updating system package repository..."
yum update -q
printf "\t[OK]\n"

# install OpenJDK 11
printf "installing OpenJDK 11..."
yum install -y -q java-11-openjdk
printf "\t[OK]\n"

# install PostgreSQL 
printf "installing PostgreSQL..."
yum install -y -q postgresql-server postgreseql-contrib
printf "\t[OK]\n"

# initialize database
printf "creating PostgreSQL service:\n"
postgresql-setup initdb
systemctl start postgresql
systemctl enable postgresql
printf "\t[OK]\n"
