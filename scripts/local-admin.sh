# Use this to insert an admin API key in your local stack and
# set that key to be used with cargo-cyndra requests
#
# Usage:
#     source scripts/local-admin.sh

key="dh9z58jttoes3qvt" # arbitrary test key
export cyndra_API_KEY=$key
docker compose --file docker-compose.rendered.yml --project-name cyndra-dev exec auth /usr/local/bin/service --state=/var/lib/cyndra-auth init-admin --name admin --key $key
