#!/usr/bin/env bash

# set -x # Enable Debug
set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
echo >&2 "Error: docker is not installed."
exit 1
fi


# Allow to skip Docker if a dockerized Postgres database is already running
container_id=$(docker ps -aqf "name=db");
if [ "$container_id" != "" ]
then
  docker stop db;
  docker rm db;
  echo "Container stoped and removed"

  /bin/bash ./scripts/init_db.sh;
else
  /bin/bash ./scripts/init_db.sh;
fi
