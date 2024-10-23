#! /usr/bin/env sh

set -ue

# Prepare directory
mkdir -p /tmp/qa-linux
cd /tmp/qa-linux

# Init app
cargo cyndra init --name qa-linux --axum

# Start locally
cargo cyndra run &
sleep 70

echo "Testing local hello endpoint"
output=$(curl --silent localhost:8000/hello)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

killall cargo-cyndra

cargo cyndra project start

cargo cyndra deploy --allow-dirty

echo "Testing remote hello endpoint"
output=$(curl --silent https://qa-linux.unstable.cyndraapp.rs/hello)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

cargo cyndra project stop

exit 0
