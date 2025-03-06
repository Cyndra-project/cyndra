# Use this to insert an admin API key in your local stack and
# set that key to be used with cargo-cyndra requests
#
# Usage:
#     source scripts/local-admin.sh

key="dh9z58jttoes3qvt" # arbitrary test key
export cyndra_API_KEY=$key
export cyndra_API="http://localhost:8001"
unset cyndra_BETA
export PS1="(cyndra: local admin key) $(echo $PS1 | sed -e "s/(cyndra: .*) //")"

docker compose --file docker-compose.rendered.yml --project-name cyndra-dev exec auth /usr/local/bin/cyndra-auth --db-connection-uri=postgres://postgres:postgres@control-db init-admin --user-id admin --key $key
