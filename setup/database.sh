#!/usr/bin/env bash

# ensure root user is being used
if [ $EUID -ne 0 ]; then
    echo "This script must be run as root."
    exit 1
fi

# set environment variables
BASEDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# make database scripts accessible
printf "Preparing database scripts..."
cp -r $BASEDIR/../src/database/ /tmp/
chmod -R 777 /tmp/database
printf "\t[OK]\n"

# create database user
printf "Creating et_mm database user...\n"
su - postgres -c "psql -d template1 -a -w -f /tmp/database/create_user.pgsql"

# create database
printf "Creating et_mm database...\n"
su - postgres -c "psql -a -w -f /tmp/database/create_database.pgsql"

# configure databse peer authentication
print "Configuring peer authnetication for et_mm database..."
if !(grep -Fxq "et_mm et_mm et_mm" /var/lib/pgsql/data/pg_ident.conf); then
    echo "et_mm et_mm et_mm" >> /var/lib/pgsql/data/pg_ident.conf
fi
if !(grep -Fxq "local all et_mm peer map=et_mm" /var/lib/pgsql/data/pg_hba.conf); then
    echo "local all et_mm peer map=et_mm" >> /var/lib/pgsql/data/pg_hba.conf
fi
printf "\t[OK]\n"

# restart postgresql
printf "Restarting postgresql service..."
systemctl restart postgresql
printf "\t[OK]\n"

# cleanup
printf "Cleaning up database scripts..."
rm -rf /tmp/database
printf "\t[OK]\n"
