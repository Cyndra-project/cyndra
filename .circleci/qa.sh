#! /usr/bin/env sh

set -ue

# Prepare directory
mkdir -p /tmp/qa-$1
cd /tmp/qa-$1

# Init app
cargo cyndra init --name qa-$1 --axum

# Start locally
cargo cyndra run &
sleep 150

echo "Testing local hello endpoint"
output=$(curl --silent localhost:8000)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

killall cargo-cyndra

cargo cyndra project start

cargo cyndra deploy --allow-dirty

echo "Testing remote hello endpoint"
output=$(curl --silent https://qa-$1.unstable.cyndraapp.rs)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

cargo cyndra project stop

exit 0
