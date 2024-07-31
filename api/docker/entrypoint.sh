#!/usr/bin/env bash

if [ -z $PROXY_FQDN ]
then
    echo "The variable 'PROXY_FQDN' is missing"
    exit 1
fi

if [ -z $PROVISIONER_ADDRESS ]
then
    echo "The variable 'PROVISIONER_ADDRESS' is missing"
    exit 1
fi

export CRATES_PATH=${CRATES_PATH:-/var/lib/cyndra/crates}

mkdir -p $CRATES_PATH

export PROXY_PORT=${PROXY_PORT:-8000}

export API_PORT=${API_PORT:-8001}

if [[ ! -z "${cyndra_USERS_TOML}" && ! -s "${cyndra_USERS_TOML}" ]]
then
    if [[ -z "${cyndra_INITIAL_KEY}" ]]
    then
        echo "\$cyndra_INITIAL_KEY is not set to create initial user's key"
        exit 1
    fi

    echo "Creating a first user with key '${cyndra_INITIAL_KEY}' at '${cyndra_USERS_TOML}'"
    mkdir -p $(dirname "${cyndra_USERS_TOML}")
    echo -e "[$cyndra_INITIAL_KEY]\nname = 'first-user'\nprojects = []" > "${cyndra_USERS_TOML}"
fi

exec supervisord -n -c /usr/share/supervisord/supervisord.conf
