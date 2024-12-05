#!/usr/bin/env bash
cargo test -p cargo-cyndra --all-features --test '*' -- needs_docker --nocapture
