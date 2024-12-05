#!/usr/bin/env bash
cargo test -p cyndra-logger --all-features --test '*' -- needs_docker --nocapture
