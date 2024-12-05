#!/usr/bin/env bash
cargo test -p cyndra-service --all-features --test '*' -- --skip needs_docker --nocapture
