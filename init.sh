#!/bin/bash

if [[ -z $POSTGRES_PASSWORD ]]; then
        echo -n "Set postgres password: "
        IFS= read -rs POSTGRES_PASSWORD < /dev/tty
        echo
fi

podman stop snackotron-psql 2> /dev/null
podman run --rm -d --name snackotron-psql -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -e POSTGRES_DB=snackotron -p 5432:5432 docker.io/postgres:latest

sleep 1

DB_URL="postgres://postgres:$POSTGRES_PASSWORD@localhost/snackotron"

psql $DB_URL < snackotron.db.sql

# create the .env file if it doesn't exist
if [[ ! -f .env ]]; then
        echo -e "DATABASE_URL=$DB_URL\nUPC_TOKEN=\n" > .env
fi
