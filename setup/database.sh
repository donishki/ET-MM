#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

#  create database user
printf "creating et_mm database user..."
su - postgres -c "psql \"CREATE USER et_mm;\""
printf "\t[OK]\n"

# create database
printf "creating et_mm database..."
su - postgres -c "psql \"CREATE DATABASE et_mm_db OWNER et_mm;\""
su - postgres -c "psql \"GRANT CONNECT ON DATABASE et_mm_db TO et_mm;\"" 
printf "\t[OK]\n"

# configure databse peer authentication
print "configuring peer authnetication for et_mm database..."
echo "et_mm et_mm et_mm" >> /etc/postgresql/12/main/pg_ident.conf
echo "local all et_mm peer map=et_mm" >> /etc/postgresql/12/main/pg_hba.conf

# restart postgresql
systemctl restart postgresql
