#!/usr/bin/env bash
cargo test -p cyndra-auth --all-features --test '*' -- needs_docker --nocapture
