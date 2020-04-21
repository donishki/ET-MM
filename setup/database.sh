#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

#  create database user
printf "Creating et_mm database user...\n"
su - postgres -c "psql -d template1 -c \"CREATE USER et_mm;\""

# create database
printf "Creating et_mm database...\n"
su - postgres -c "psql -c \"CREATE DATABASE et_mm_db OWNER et_mm;\""
su - postgres -c "psql -c \"GRANT CONNECT ON DATABASE et_mm_db TO et_mm;\"" 

# configure databse peer authentication
print "Configuring peer authnetication for et_mm database..."
echo "et_mm et_mm et_mm" >> /var/lib/pgsql/data/pg_ident.conf
echo "local all et_mm peer map=et_mm" >> /var/lib/pgsql/data/pg_hba.conf
printf "\t[OK]\n"

# restart postgresql
print "Restarting postgresql service\n"
systemctl restart postgresql
