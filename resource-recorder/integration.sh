#!/usr/bin/env bash
cargo test -p cyndra-resource-recorder --all-features --test '*' -- --skip needs_docker --nocapture
