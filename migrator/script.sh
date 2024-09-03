#!/usr/bin/env sh

scp ubuntu@18.133.52.140:/opt/cyndra/user-data/users/users.toml users.toml
cargo run -- users.toml > users.sql


scp users.sql controller.cyndra.internal:~/users.sql
ssh controller.cyndra.internal "cat ~/users.sql | sudo sqlite3 /var/lib/docker/volumes/cyndra-dev_gateway-vol/_data/gateway.sqlite"
