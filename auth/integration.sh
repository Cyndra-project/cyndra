#!/usr/bin/env bash
cargo test -p cyndra-auth --all-features --test '*' -- --skip needs_docker --nocapture
