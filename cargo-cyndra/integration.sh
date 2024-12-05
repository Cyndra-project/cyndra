#!/usr/bin/env bash
cargo test -p cargo-cyndra --all-features --test '*' -- --skip needs_docker --nocapture
